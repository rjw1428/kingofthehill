import { ChangeDetectionStrategy, Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatSelectModule } from '@angular/material/select';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { BackendService } from '../services/backend.service';
import { AbstractControl, FormControl, ReactiveFormsModule } from '@angular/forms'
import { AuthService } from '../services/auth.service';
import { take, switchMap } from 'rxjs';

@Component({
  standalone: true,
  imports: [
    CommonModule,
    MatButtonModule,
    MatSelectModule,
    MatFormFieldModule,
    MatInputModule,
    ReactiveFormsModule,
  ],
  template: `
    <div class="home-header">
      <h1>Home</h1>
      <h3 *ngIf="user$ | async as user">{{ user.name }}</h3>
    </div>
    <h3>Team Selector</h3>
    <mat-form-field>
      <mat-select placeholder="Select Team" [formControl]="form">
        <mat-option *ngFor="let team of teams$ | async" [value]="team._id!.$oid">
          {{ team.city }} {{ team.name }}
        </mat-option>
      </mat-select>
    </mat-form-field>
    <button mat-flat-button color="primary" (click)="submit(form)">Submit</button>

    <pre>{{ picks$ | async | json }}</pre>
  `,
  styleUrls: ['./home.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class HomeComponent {
  teams$ = this.backend.getTeams();
  user$ = this.authService.getUser();
  picks$ = this.backend.getPicks();
  form = new FormControl() 
  constructor(
    private backend: BackendService,
    private authService: AuthService,
  ) {}

  submit(form: AbstractControl) {
    return this.user$.pipe(
      take(1),
      switchMap(user => this.backend.submitPick({
        userId: user.id,
        teamId: form.value,
        weekId: "2023-1",
        leagueId: user.leagueId,
      }))
    ).subscribe({
      next: resp => console.log(resp),
      error: err => console.warn(err)
    })
  }
}
