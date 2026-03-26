import type { CSSProperties } from 'react';
import { Dropdown, Option, Label } from '@fluentui/react-components';
import type { SelectionEvents, OptionOnSelectData } from '@fluentui/react-components';

interface ScheduleOption {
  value: string;
  label: string;
}

const scheduleOptions: ScheduleOption[] = [
  { value: '30', label: '30 phút' },
  { value: '60', label: '1 giờ' },
  { value: '90', label: '90 phút' },
  { value: '1440', label: '1 ngày' },
];

interface ScheduleSelectProps {
  id: string;
  label: string;
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
  required?: boolean;
}

export function ScheduleSelect({
  id,
  label,
  value,
  onChange,
  disabled = false,
  required = false,
}: ScheduleSelectProps) {
  const handleChange = (_event: SelectionEvents, data: OptionOnSelectData) => {
    onChange(data.optionValue as string);
  };

  return (
    <div style={styles.container}>
      <Label 
        htmlFor={`${id}-select`}
        required={required}
        style={styles.label}
      >
        {label}
      </Label>
      <Dropdown
        id={`${id}-select`}
        value={scheduleOptions.find(opt => opt.value === value)?.label || 'Chọn lịch...'}
        selectedOptions={[value]}
        onOptionSelect={handleChange}
        disabled={disabled}
        style={styles.dropdown}
      >
        {scheduleOptions.map((option) => (
          <Option
            key={option.value}
            value={option.value}
            aria-label={option.label}
          >
            {option.label}
          </Option>
        ))}
      </Dropdown>
    </div>
  );
}

const styles: Record<string, CSSProperties> = {
  container: {
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    marginBottom: '16px',
  },
  label: {
    fontWeight: 500,
    color: '#1f1f1f',
  },
  dropdown: {
    width: '200px',
  },
};

export default ScheduleSelect;
