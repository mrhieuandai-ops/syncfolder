import { BoardRegular, PersonRegular, ClockRegular, SettingsRegular } from '@fluentui/react-icons';
import type { NavItem } from './AppShell';

export interface NavItemConfig {
  key: NavItem;
  label: string;
  icon: React.ReactElement;
}

export const navItems: NavItemConfig[] = [
  {
    key: 'dashboard',
    label: 'Điều khiển',
    icon: <BoardRegular />,
  },
  {
    key: 'profiles',
    label: 'Hồ sơ',
    icon: <PersonRegular />,
  },
  {
    key: 'activity-log',
    label: 'Nhật ký hoạt động',
    icon: <ClockRegular />,
  },
  {
    key: 'settings',
    label: 'Cài đặt',
    icon: <SettingsRegular />,
  },
];
