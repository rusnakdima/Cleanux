export interface TemperatureInfo {
  name: string;
  sensor_type: 'cpu' | 'gpu';
  temperature_celsius: number;
  max_temp: number;
}
