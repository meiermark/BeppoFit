import { ComponentFixture, TestBed } from '@angular/core/testing';
import { AccountComponent } from './account.component';
import { AuthService } from '../../core/auth/auth.service';
import { Router } from '@angular/router';
import { of, throwError } from 'rxjs';
import { RouterTestingModule } from '@angular/router/testing';

describe('AccountComponent', () => {
    let component: AccountComponent;
    let fixture: ComponentFixture<AccountComponent>;
    let authServiceSpy: jasmine.SpyObj<AuthService>;
    let routerSpy: jasmine.SpyObj<Router>;

    beforeEach(async () => {
        authServiceSpy = jasmine.createSpyObj('AuthService', ['deleteAccount', 'logout']);
        routerSpy = jasmine.createSpyObj('Router', ['navigate']);

        // Default spy returns
        authServiceSpy.deleteAccount.and.returnValue(of(void 0));

        await TestBed.configureTestingModule({
            imports: [AccountComponent, RouterTestingModule],
            providers: [
                { provide: AuthService, useValue: authServiceSpy },
                // Router provided by wrapper, or verify stub usage
            ]
        })
            .compileComponents();

        fixture = TestBed.createComponent(AccountComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it('should create', () => {
        expect(component).toBeTruthy();
    });

    it('should call deleteAccount service when confirmed', () => {
        spyOn(window, 'confirm').and.returnValue(true);
        spyOn(window, 'alert'); // Suppress alert

        component.deleteAccount();

        expect(window.confirm).toHaveBeenCalled();
        expect(authServiceSpy.deleteAccount).toHaveBeenCalled();
        expect(authServiceSpy.logout).toHaveBeenCalled(); // Should logout after success
    });

    it('should NOT call deleteAccount service when cancelled', () => {
        spyOn(window, 'confirm').and.returnValue(false);

        component.deleteAccount();

        expect(window.confirm).toHaveBeenCalled();
        expect(authServiceSpy.deleteAccount).not.toHaveBeenCalled();
    });
});
