/* angular */
import { Component, inject, ChangeDetectionStrategy, OnDestroy, OnInit } from '@angular/core';
import { Router, NavigationEnd } from '@angular/router';
import { filter } from 'rxjs/operators';
import { Subscription } from 'rxjs';

/* library */
import {
  SchemaRouterService,
  SchemaRouteViewerComponent,
  SchemaSetupService,
} from '@tauri-front/shared';

@Component({
  selector: 'app-root',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [SchemaRouteViewerComponent],
  templateUrl: './app.html',
})
export class App implements OnInit, OnDestroy {
  private router = inject(Router);
  private routerSubscription!: Subscription;
  protected schemaRouter = inject(SchemaRouterService);
  private setup = inject(SchemaSetupService);

  constructor() {}

  ngOnInit() {
    this.routerSubscription = this.router.events
      .pipe(filter((event) => event instanceof NavigationEnd))
      .subscribe((event: NavigationEnd) => {
        this.schemaRouter.navigate(event.urlAfterRedirects);
      });

    this.setup.setup('cleanux', {
      initialRoute: '/dashboard',
      autoRegisterRoutes: true,
    });
  }

  ngOnDestroy(): void {
    this.routerSubscription?.unsubscribe();
  }
}
