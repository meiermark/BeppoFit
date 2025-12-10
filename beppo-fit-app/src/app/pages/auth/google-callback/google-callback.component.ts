import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { AuthService } from '../../../core/auth/auth.service';

@Component({
    selector: 'app-google-callback',
    template: '<ion-content class="ion-padding"><div class="ion-text-center">Processing Login...</div></ion-content>',
    standalone: false
})
export class GoogleCallbackComponent implements OnInit {

    constructor(
        private route: ActivatedRoute,
        private router: Router,
        private authService: AuthService
    ) { }

    ngOnInit() {
        // This expects the token to be in query params OR fragment.
        // Since we are changing backend to redirect, let's assume query params for now.
        this.route.queryParams.subscribe(params => {
            const token = params['token'];
            if (token) {
                // Backend didn't send user object in query param (security/length), so we might need to fetch it.
                // But for now, let's decode token or just assume it's valid.
                // AuthService expects user object.
                // We'll decode JWT to get user ID, or fetch /me.
                // Let's create a partial user object or fetch it.
                // For MVP, if token is present, we consider logged in.
                // We'll update handleGoogleCallback to just take token and decode/fetch.
                this.authService.handleGoogleCallback(token, { id: 'temp', email: 'temp', is_verified: true }); // Temp user
                this.router.navigate(['/home']);
            } else {
                // Handle error
                this.router.navigate(['/auth/login']);
            }
        });
    }
}
