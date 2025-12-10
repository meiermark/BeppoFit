import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { AuthService } from '../../../core/auth/auth.service';
import { ToastController } from '@ionic/angular';

@Component({
    selector: 'app-forgot-password',
    templateUrl: './forgot-password.component.html',
    styleUrls: ['./forgot-password.component.scss'],
    standalone: false
})
export class ForgotPasswordComponent implements OnInit {
    forgotForm: FormGroup;

    constructor(
        private fb: FormBuilder,
        private authService: AuthService,
        private toastController: ToastController
    ) {
        this.forgotForm = this.fb.group({
            email: ['', [Validators.required, Validators.email]]
        });
    }

    ngOnInit() { }

    async onSubmit() {
        if (this.forgotForm.valid) {
            this.authService.forgotPassword(this.forgotForm.value.email).subscribe({
                next: async () => {
                    const toast = await this.toastController.create({
                        message: 'If an account exists, a reset email has been sent.',
                        duration: 3000,
                        color: 'success'
                    });
                    toast.present();
                },
                error: async () => {
                    const toast = await this.toastController.create({
                        message: 'An error occurred. Please try again.',
                        duration: 3000,
                        color: 'danger'
                    });
                    toast.present();
                }
            });
        }
    }
}
