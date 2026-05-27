import { Component, inject, signal, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatSlideToggleModule } from '@angular/material/slide-toggle';
import { MatSelectModule } from '@angular/material/select';
import { MatDialogModule, MatDialog } from '@angular/material/dialog';
import {
  AutomationService,
  QuickAction,
  AutomationRecipe,
  ActionStep,
  ExecutionHistoryEntry,
} from '@services/automation.service';
import { ProfileService } from '@services/profile.service';
import { HeaderComponent } from '@components/header/header.component';
import { CleaningProfile } from '@models/profile.model';

@Component({
  selector: 'app-automation',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatIconModule,
    MatButtonModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatSlideToggleModule,
    MatSelectModule,
    MatDialogModule,
    HeaderComponent,
  ],
  templateUrl: './automation.view.html',
  styleUrl: './automation.view.css',
})
export class AutomationView implements OnInit {
  private automationService = inject(AutomationService);
  private profileService = inject(ProfileService);
  private dialog = inject(MatDialog);

  quickActions = signal<QuickAction[]>([]);
  recipes = signal<AutomationRecipe[]>([]);
  history = signal<ExecutionHistoryEntry[]>([]);
  profiles = signal<CleaningProfile[]>([]);
  isLoading = signal(false);
  activeTab = signal<'quick' | 'recipes' | 'history'>('quick');
  isBuilderOpen = signal(false);
  editingRecipe = signal<AutomationRecipe | null>(null);

  newRecipeName = signal('');
  newRecipeSteps = signal<ActionStep[]>([]);
  newRecipeTrigger = signal<'Manual' | 'Scheduled' | 'Event'>('Manual');

  availableCategories = [
    { value: 'cache', label: 'Cache' },
    { value: 'trash', label: 'Trash' },
    { value: 'logs', label: 'Logs' },
    { value: 'largefiles', label: 'Large Files' },
  ];

  async ngOnInit() {
    await this.loadData();
  }

  async loadData() {
    this.isLoading.set(true);
    try {
      const [actions, recipes, history, profiles] = await Promise.all([
        this.automationService.getQuickActions(),
        this.automationService.getRecipes(),
        this.automationService.getExecutionHistory(),
        this.profileService.listProfiles(),
      ]);
      this.quickActions.set(actions);
      this.recipes.set(recipes);
      this.history.set(history);
      this.profiles.set(profiles);
    } catch (e) {
      console.error('Failed to load automation data:', e);
    } finally {
      this.isLoading.set(false);
    }
  }

  async executeQuickAction(action: QuickAction) {
    if (!confirm(`Execute "${action.name}"?`)) return;
    this.isLoading.set(true);
    try {
      await this.automationService.executeAction(action.id);
      await this.loadData();
      alert(`"${action.name}" executed successfully`);
    } catch (e) {
      alert('Failed to execute action: ' + (e instanceof Error ? e.message : String(e)));
    } finally {
      this.isLoading.set(false);
    }
  }

  async executeRecipe(recipe: AutomationRecipe) {
    if (!confirm(`Execute recipe "${recipe.name}"?`)) return;
    this.isLoading.set(true);
    try {
      await this.automationService.executeRecipe(recipe.id);
      await this.loadData();
      alert(`Recipe "${recipe.name}" executed successfully`);
    } catch (e) {
      alert('Failed to execute recipe: ' + (e instanceof Error ? e.message : String(e)));
    } finally {
      this.isLoading.set(false);
    }
  }

  async toggleRecipe(recipe: AutomationRecipe) {
    const updated = { ...recipe, enabled: !recipe.enabled };
    try {
      await this.automationService.saveRecipe(updated);
      await this.loadData();
    } catch (e) {
      alert('Failed to update recipe');
    }
  }

  async deleteRecipe(recipe: AutomationRecipe) {
    if (!confirm(`Delete recipe "${recipe.name}"?`)) return;
    this.isLoading.set(true);
    try {
      await this.automationService.deleteRecipe(recipe.id);
      await this.loadData();
    } catch (e) {
      alert('Failed to delete recipe');
    } finally {
      this.isLoading.set(false);
    }
  }

  openRecipeBuilder(recipe?: AutomationRecipe) {
    if (recipe) {
      this.editingRecipe.set({ ...recipe });
      this.newRecipeName.set(recipe.name);
      this.newRecipeSteps.set([...recipe.steps]);
      this.newRecipeTrigger.set(recipe.trigger);
    } else {
      this.editingRecipe.set(null);
      this.newRecipeName.set('');
      this.newRecipeSteps.set([]);
      this.newRecipeTrigger.set('Manual');
    }
    this.isBuilderOpen.set(true);
  }

  closeRecipeBuilder() {
    this.isBuilderOpen.set(false);
    this.editingRecipe.set(null);
    this.newRecipeName.set('');
    this.newRecipeSteps.set([]);
    this.newRecipeTrigger.set('Manual');
  }

  addStep(type: 'CleanCategory' | 'RunProfile' | 'ExecuteCommand' | 'Wait', value: any) {
    let step: ActionStep;
    switch (type) {
      case 'CleanCategory':
        step = { CleanCategory: { category: value } };
        break;
      case 'RunProfile':
        step = { RunProfile: { profileName: value } };
        break;
      case 'ExecuteCommand':
        step = { ExecuteCommand: { command: value } };
        break;
      case 'Wait':
        step = { Wait: { seconds: value } };
        break;
      default:
        return;
    }
    this.newRecipeSteps.update((steps) => [...steps, step]);
  }

  removeStep(index: number) {
    this.newRecipeSteps.update((steps) => steps.filter((_, i) => i !== index));
  }

  moveStep(index: number, direction: 'up' | 'down') {
    const steps = this.newRecipeSteps();
    if (direction === 'up' && index > 0) {
      [steps[index - 1], steps[index]] = [steps[index], steps[index - 1]];
    } else if (direction === 'down' && index < steps.length - 1) {
      [steps[index], steps[index + 1]] = [steps[index + 1], steps[index]];
    }
    this.newRecipeSteps.set([...steps]);
  }

  async saveRecipe() {
    const name = this.newRecipeName();
    if (!name.trim()) {
      alert('Recipe name is required');
      return;
    }
    if (this.newRecipeSteps().length === 0) {
      alert('At least one step is required');
      return;
    }

    const recipe: AutomationRecipe = {
      id: this.editingRecipe()?.id || '',
      name,
      steps: this.newRecipeSteps(),
      enabled: this.editingRecipe()?.enabled ?? true,
      trigger: this.newRecipeTrigger(),
    };

    this.isLoading.set(true);
    try {
      await this.automationService.saveRecipe(recipe);
      await this.loadData();
      this.closeRecipeBuilder();
      alert('Recipe saved successfully');
    } catch (e) {
      alert('Failed to save recipe');
    } finally {
      this.isLoading.set(false);
    }
  }

  getStepDescription(step: ActionStep): string {
    if (step.CleanCategory) return `Clean ${step.CleanCategory.category}`;
    if (step.RunProfile) return `Run profile: ${step.RunProfile.profileName}`;
    if (step.ExecuteCommand) return `Execute: ${step.ExecuteCommand.command}`;
    if (step.Wait) return `Wait ${step.Wait.seconds}s`;
    return 'Unknown step';
  }

  getStepIcon(step: ActionStep): string {
    if (step.CleanCategory) return 'cleaning_services';
    if (step.RunProfile) return 'play_arrow';
    if (step.ExecuteCommand) return 'terminal';
    if (step.Wait) return 'schedule';
    return 'drag_handle';
  }

  getActionIcon(action: QuickAction): string {
    return action.icon || 'flash_on';
  }

  formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleString();
  }

  formatDuration(startedAt: string, completedAt?: string): string {
    if (!completedAt) return 'Running...';
    const start = new Date(startedAt).getTime();
    const end = new Date(completedAt).getTime();
    const seconds = Math.round((end - start) / 1000);
    if (seconds < 60) return `${seconds}s`;
    return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
  }
}
