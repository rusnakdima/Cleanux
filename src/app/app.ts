/* sys lib */
import {
  Component,
  signal,
  inject,
  ChangeDetectionStrategy,
  OnDestroy,
  OnInit,
} from '@angular/core';
import { Router, NavigationEnd } from '@angular/router';
import { filter } from 'rxjs/operators';
import { Subscription } from 'rxjs';

/* shared */
import {
  SchemaRouterService,
  SchemaRouteViewerComponent,
  InvokeWrapperService,
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
  protected invoke = inject(InvokeWrapperService);

  constructor() {}

  ngOnInit() {
    this.routerSubscription = this.router.events
      .pipe(filter((event) => event instanceof NavigationEnd))
      .subscribe((event: NavigationEnd) => {
        this.schemaRouter.navigate(event.urlAfterRedirects);
      });

    this.initSchema();
  }

  private async initSchema() {
    try {
      const response = await this.invoke.invoke<any>('get_ui_schema', { id: 'cleanux' });
      const schema = response?.data ?? response;
      if (schema?.pages?.length) {
        this.schemaRouter.setSchema(schema);
        this.schemaRouter.navigate('/dashboard');
      }
    } catch (e) {
      throw e;
    }
  }

  ngOnDestroy(): void {
    this.routerSubscription?.unsubscribe();
  }
}
