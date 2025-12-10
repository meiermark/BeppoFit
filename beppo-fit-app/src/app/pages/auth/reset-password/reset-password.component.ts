import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { AuthService } from '../../../core/auth/auth.service';
import { ToastController } from '@ionic/angular';

@Component({
    selector: 'app-reset-password',
    templateUrl: './reset-password.component.html',
    styleUrls: ['./reset-password.component.scss'],
    standalone: false
})
export class ResetPasswordComponent implements OnInit {
    resetForm: FormGroup;
    token: string = '';

    constructor(
        private fb: FormBuilder,
        private authService: AuthService,
        private route: ActivatedRoute,
        private router: Router,
        private toastController: ToastController
    ) {
        this.resetForm = this.fb.group({
            password: ['', [Validators.required, Validators.minLength(8)]]
        });
    }

    ngOnInit() {
        this.token = this.route.snapshot.queryParams['token'];
    }

    async onSubmit() {
        if (this.resetForm.valid && this.token) {
            this.authService.resetPassword(this.token, this.resetForm.value.password).subscribe({
                next: async () => {
                    const toast = await this.toastController.create({
                        message: 'Password has been reset. Please login.',
                        duration: 3000,
                        color: 'success'
                    });
                    toast.present();
                    this.router.navigate(['/auth/login']);
                },
                error: async (err: any) => {
                    const toast = await this.toastController.create({
                        message: err.error?.error || 'Failed to reset password.',
                        duration: 3000,
                        color: 'danger'
                    });
                    toast.present();
                }
            });
        }
    }
}
