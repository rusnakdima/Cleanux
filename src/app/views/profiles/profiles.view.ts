import { Component, inject, signal, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { ProfileService } from '@services/profile.service';
import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';
import { ToastService } from '@shared/toast';
import { CleaningProfile, createEmptyProfile } from '@models/profile.model';
import { formatSize } from '@shared/utils/format.util';

@Component({
  selector: 'app-profiles',
  standalone: true,
  imports: [CommonModule, FormsModule, MatIconModule, MatProgressSpinnerModule, MatTooltipModule],
  templateUrl: './profiles.view.html',
})
export class ProfilesView implements OnInit {
  private profileService = inject(ProfileService);
  private notification = inject(NotificationService);
  private confirmDialogService = inject(ConfirmDialogService);
  private toastService = inject(ToastService);

  profiles = signal<CleaningProfile[]>([]);
  isLoading = signal(false);
  isEditing = signal(false);
  editingProfile = signal<CleaningProfile | null>(null);
  selectedProfile = signal<CleaningProfile | null>(null);

  async ngOnInit() {
    await this.loadProfiles();
  }

  async loadProfiles() {
    this.isLoading.set(true);
    try {
      const profiles = await this.profileService.listProfiles();
      this.profiles.set(profiles);
    } catch (e) {
      console.error('Failed to load profiles:', e);
    } finally {
      this.isLoading.set(false);
    }
  }

  newProfile() {
    this.editingProfile.set(createEmptyProfile());
    this.isEditing.set(true);
    this.selectedProfile.set(null);
  }

  editProfile(profile: CleaningProfile) {
    this.editingProfile.set({ ...profile });
    this.isEditing.set(true);
    this.selectedProfile.set(profile);
  }

  cancelEdit() {
    this.editingProfile.set(null);
    this.isEditing.set(false);
  }

  async saveProfile() {
    const profile = this.editingProfile();
    if (!profile || !profile.name.trim()) {
      this.toastService.show('Profile name is required', 'warning');
      return;
    }

    this.isLoading.set(true);
    try {
      await this.profileService.saveProfile(profile);
      await this.loadProfiles();
      this.cancelEdit();
      this.notification.success('Profile saved successfully');
    } catch (e) {
      this.notification.error('Failed to save profile', e);
    } finally {
      this.isLoading.set(false);
    }
  }

  async deleteProfile(profile: CleaningProfile) {
    if (!await this.confirmDialogService.confirm({
      title: 'Delete Profile',
      message: `Delete profile "${profile.name}"?`,
      dangerous: true,
    })) return;

    this.isLoading.set(true);
    try {
      await this.profileService.deleteProfile(profile.name);
      await this.loadProfiles();
      if (this.selectedProfile()?.name === profile.name) {
        this.selectedProfile.set(null);
      }
    } catch (e) {
      this.notification.error('Failed to delete profile', e);
    } finally {
      this.isLoading.set(false);
    }
  }

  async applyProfile(profile: CleaningProfile) {
    if (!await this.confirmDialogService.confirm({
      title: 'Apply Profile',
      message: `Apply profile "${profile.name}"?`,
    })) return;

    this.isLoading.set(true);
    try {
      const result = await this.profileService.applyProfile(profile.name);
      this.notification.success(`Profile applied: ${result}`);
    } catch (e) {
      this.notification.error('Failed to apply profile', e);
    } finally {
      this.isLoading.set(false);
    }
  }

  exportProfile(profile: CleaningProfile) {
    this.profileService.exportProfile(profile);
  }

  async importProfile(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    this.isLoading.set(true);
    try {
      const profile = await this.profileService.importProfile(file);
      profile.name = `${profile.name} (imported)`;
      await this.profileService.saveProfile(profile);
      await this.loadProfiles();
      this.notification.success('Profile imported successfully');
    } catch (e) {
      this.notification.error('Failed to import profile', e);
    } finally {
      this.isLoading.set(false);
      input.value = '';
    }
  }

  onPathsChange(event: Event) {
    const input = event.target as HTMLInputElement;
    const profile = this.editingProfile();
    if (profile) {
      profile.paths = input.value
        .split('\n')
        .map((p) => p.trim())
        .filter((p) => p);
    }
  }

  onExcludePatternsChange(event: Event) {
    const input = event.target as HTMLInputElement;
    const profile = this.editingProfile();
    if (profile) {
      profile.exclude_patterns = input.value
        .split('\n')
        .map((p) => p.trim())
        .filter((p) => p);
      this.editingProfile.set(profile);
    }
  }

  formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleDateString();
  }

  formatBytes(bytes: number): string {
    return formatSize(bytes);
  }
}
