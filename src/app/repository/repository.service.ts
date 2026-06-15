import { Injectable, inject } from '@angular/core';
import { Observable, of, tap, catchError } from 'rxjs';
import { ApiService } from '../services/api.service';
import { CacheService } from './cache.service';

export interface Repository<T, ID = string> {
  getAll(): Observable<T[]>;
  getById(id: ID): Observable<T | null>;
  create(data: Partial<T>): Observable<T>;
  update(id: ID, data: Partial<T>): Observable<T>;
  delete(id: ID): Observable<void>;
}

@Injectable({ providedIn: 'root' })
export abstract class BaseRepository<T, ID = string> implements Repository<T, ID> {
  protected api = inject(ApiService);
  protected cache = inject(CacheService);
  
  protected abstract cacheKey(id?: ID): string;
  protected abstract commandGetAll: string;
  protected abstract commandGetById: string;
  protected abstract commandCreate: string;
  protected abstract commandUpdate: string;
  protected abstract commandDelete: string;

  getAll(): Observable<T[]> {
    const key = this.cacheKey();
    const cached = this.cache.get<T[]>(key);
    if (cached) return of(cached);

    return this.api.invoke<T[]>(this.commandGetAll, {}).pipe(
      tap(items => this.cache.set(key, items)),
      catchError(err => {
        console.error(`Failed to get all: ${this.commandGetAll}`, err);
        return of([]);
      })
    );
  }

  getById(id: ID): Observable<T | null> {
    const key = this.cacheKey(id);
    const cached = this.cache.get<T>(key);
    if (cached) return of(cached);

    return this.api.invoke<T | null>(this.commandGetById, { id }).pipe(
      tap(item => { if (item) this.cache.set(key, item); }),
      catchError(() => of(null))
    );
  }

  create(data: Partial<T>): Observable<T> {
    return this.api.invoke<T>(this.commandCreate, { data }).pipe(
      tap(() => this.cache.invalidatePrefix(this.baseKey()))
    );
  }

  update(id: ID, data: Partial<T>): Observable<T> {
    return this.api.invoke<T>(this.commandUpdate, { id, data }).pipe(
      tap(item => {
        this.cache.invalidatePrefix(this.baseKey());
        this.cache.set(this.cacheKey(id), item);
      })
    );
  }

  delete(id: ID): Observable<void> {
    return this.api.invoke<void>(this.commandDelete, { id }).pipe(
      tap(() => {
        this.cache.invalidatePrefix(this.baseKey());
        this.cache.delete(this.cacheKey(id));
      })
    );
  }

  protected baseKey(): string {
    return '';
  }
}
