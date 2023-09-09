import { Injectable } from '@angular/core';
import { of } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  getUser() {
    return of({
      name: "Ryan",
      id: 'abc123',
      leagueId: 'league1'
    })
  }
}
