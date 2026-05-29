import { inject } from '@angular/core';
import { CanActivateFn, Router } from '@angular/router';

export const authGuard: CanActivateFn = async () => {
  const router = inject(Router);
  return true;
};

export const adminGuard: CanActivateFn = async () => {
  const router = inject(Router);
  return true;
};

export const unsavedChangesGuard: CanActivateFn = async () => {
  const router = inject(Router);
  return true;
};
