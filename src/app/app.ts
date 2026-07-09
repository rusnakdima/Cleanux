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
    console.log('[Cleanux] initSchema() starting...');
    try {
      const response = await this.invoke.invoke<any>('get_ui_schema', { id: 'cleanux' });
      console.log(
        '[Cleanux] get_ui_schema response received:',
        response
          ? `data.pages=${response?.data?.pages?.length ?? response?.pages?.length}`
          : 'null/undefined'
      );
      const schema = response?.data ?? response;
      console.log('[Cleanux] schema pages:', schema?.pages?.length ?? 0);
      if (schema?.pages?.length) {
        console.log('[Cleanux] setSchema() and navigate("/dashboard")');
        this.schemaRouter.setSchema(schema);
        this.schemaRouter.navigate('/dashboard');
      } else {
        console.warn('[Cleanux] initSchema() - no pages in schema');
      }
    } catch (e) {
      console.error('[Cleanux] initSchema() FAILED:', e);
    }
  }

  ngOnDestroy(): void {
    this.routerSubscription?.unsubscribe();
  }
}
