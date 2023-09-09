import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, catchError, of } from 'rxjs';
import { environment } from '../../environments/environment';
import { Pick } from 'libs/shared/pick'
import { Team } from 'libs/shared/team'
import { Response } from 'libs/shared/response'

@Injectable({
  providedIn: 'root'
})
export class BackendService {
  private api = environment.api;
  private port = environment.port

  constructor(private http: HttpClient) { }

  getTeams(): Observable<Team[]> {
    return this.http.get<Team[]>(`${this.api}:${this.port}/api/v1/teams`).pipe(
      catchError(err => {
        console.warn(err)
        return of([])
      })
    )
  }

  getPicks(): Observable<Pick[]> {
    return this.http.get<Pick[]>(`${this.api}:${this.port}/api/v1/picks`).pipe(
      catchError(err => {
        console.warn(err)
        return of([])
      })
    )
  }

  submitPick(pick: Pick):  Observable<Response> {
    return this.http.post<Response>(`${this.api}:${this.port}/api/v1/pick`, pick)
  }
}