export interface BatteryInfo {
  present: boolean;
  charge_percent: number;
  health_percent: number;
  cycles: number;
  temperature: number;
  status: string;
}

export interface PowerProfile {
  name: string;
  profile_type: string;
  active: boolean;
}

export interface ThermalInfo {
  name: string;
  temperature_celsius: number;
  max_temp: number;
}
