import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject, Observable, tap } from 'rxjs';
import { Router } from '@angular/router';

export interface User {
    id: string;
    email: string;
    is_verified: boolean;
    google_id?: string;
    created_at?: string;
    updated_at?: string;
}

interface AuthResponse {
    token: string;
    user: User;
}

@Injectable({
    providedIn: 'root'
})
export class AuthService {
    private apiUrl = '/api/auth';
    private currentUserSubject = new BehaviorSubject<User | null>(null);
    public currentUser$ = this.currentUserSubject.asObservable();
    private tokenKey = 'beppo_auth_token';

    constructor(private http: HttpClient, private router: Router) {
        this.loadUser();
    }

    private loadUser() {
        const token = localStorage.getItem(this.tokenKey);
        if (token) {
            // In a real app, we might validate the token with the backend or decode it
            // For now, we'll assume existence means logged in, but we don't have user details
            // unless we persisted them too. Let's just persist token for now.
            // Ideally we should have a /me endpoint to fetch user details on load.
            // For this MVP, we might need to store user in localstorage too or just reliance on token.
            const storedUser = localStorage.getItem('beppo_user');
            if (storedUser) {
                this.currentUserSubject.next(JSON.parse(storedUser));
            }
        }
    }

    register(credentials: any): Observable<AuthResponse> {
        return this.http.post<AuthResponse>(`${this.apiUrl}/register`, credentials).pipe(
            tap(response => this.handleAuthResponse(response))
        );
    }

    login(credentials: any): Observable<AuthResponse> {
        return this.http.post<AuthResponse>(`${this.apiUrl}/login`, credentials).pipe(
            tap(response => this.handleAuthResponse(response))
        );
    }

    logout() {
        localStorage.removeItem(this.tokenKey);
        localStorage.removeItem('beppo_user');
        this.currentUserSubject.next(null);
        this.router.navigate(['/auth/login']);
    }

    verifyEmail(token: string): Observable<string> {
        return this.http.get<string>(`${this.apiUrl}/verify?token=${token}`);
    }

    forgotPassword(email: string): Observable<string> {
        return this.http.post<string>(`${this.apiUrl}/forgot-password`, { email });
    }

    resetPassword(token: string, newPassword: string): Observable<string> {
        return this.http.post<string>(`${this.apiUrl}/reset-password`, { token, new_password: newPassword });
    }

    googleLogin() {
        window.location.href = `${this.apiUrl}/google`;
    }

    // Called when redirecting back from Google
    handleGoogleCallback(token: string, user: User) {
        this.handleAuthResponse({ token, user });
    }

    private handleAuthResponse(response: AuthResponse) {
        localStorage.setItem(this.tokenKey, response.token);
        localStorage.setItem('beppo_user', JSON.stringify(response.user));
        this.currentUserSubject.next(response.user);
    }

    getToken(): string | null {
        return localStorage.getItem(this.tokenKey);
    }

    isAuthenticated(): boolean {
        return !!this.getToken();
    }

    deleteAccount(): Observable<any> {
        return this.http.delete(`${this.apiUrl}/me`);
    }
}
