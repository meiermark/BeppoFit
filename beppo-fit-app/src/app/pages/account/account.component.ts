import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { IonicModule } from '@ionic/angular';
import { AuthService } from '../../core/auth/auth.service';
import { Router } from '@angular/router';

@Component({
    selector: 'app-account',
    standalone: true,
    imports: [CommonModule, IonicModule],
    templateUrl: './account.component.html',
    styles: [`
    ion-content {
      --padding-start: 20px;
      --padding-end: 20px;
      --padding-top: 20px;
    }
    .danger-zone {
      margin-top: 40px;
      padding: 20px;
      border: 1px solid var(--ion-color-danger);
      border-radius: 8px;
    }
    h2 {
        margin-top: 0;
    }
  `]
})
export class AccountComponent {
    constructor(private authService: AuthService, private router: Router) { }

    deleteAccount() {
        if (confirm('Are you sure you want to delete your account? This action cannot be undone.')) {
            this.authService.deleteAccount().subscribe({
                next: () => {
                    this.authService.logout();
                    alert('Account deleted successfully.');
                },
                error: (err) => {
                    console.error('Failed to delete account', err);
                    alert('Failed to delete account. Please try again.');
                }
            });
        }
    }

    logout() {
        this.authService.logout();
    }
}
