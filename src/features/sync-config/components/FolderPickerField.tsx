import type { CSSProperties } from 'react';
import { Button, Input, Label } from '@fluentui/react-components';
import { Folder24Regular } from '@fluentui/react-icons';
import { open } from '@tauri-apps/plugin-dialog';

interface FolderPickerFieldProps {
  id: string;
  label: string;
  value: string;
  placeholder?: string;
  onPathChange: (path: string) => void;
  disabled?: boolean;
  error?: string;
  required?: boolean;
}

export function FolderPickerField({
  id,
  label,
  value,
  placeholder = 'Chọn thư mục...',
  onPathChange,
  disabled = false,
  error,
  required = false,
}: FolderPickerFieldProps) {
  
  const handleClick = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: `Chọn thư mục cho ${label.toLowerCase()}`,
      });
      
      if (selected && typeof selected === 'string') {
        onPathChange(selected);
      }
    } catch (err) {
      console.error('Failed to open folder picker:', err);
    }
  };
  return (
    <div style={styles.container}>
      <Label 
        htmlFor={`${id}-display`}
        required={required}
        style={styles.label}
      >
        {label}
      </Label>
      <div style={styles.inputWrapper}>
        <Button
          id={`${id}-button`}
          icon={<Folder24Regular />}
          onClick={handleClick}
          disabled={disabled}
          aria-label={`Chọn thư mục cho ${label.toLowerCase()}`}
          style={styles.browseButton}
        >
          Browse
        </Button>
        <Input
          id={`${id}-display`}
          value={value}
          placeholder={placeholder}
          readOnly
          disabled={disabled}
          aria-label={`Đường dẫn ${label.toLowerCase()}: ${value || 'Chưa chọn'}`}
          style={styles.input}
        />
      </div>
      {error && (
        <span 
          id={`${id}-error`}
          style={styles.error}
          role="alert"
        >
          {error}
        </span>
      )}
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
  inputWrapper: {
    display: 'flex',
    gap: '8px',
    alignItems: 'center',
  },
  browseButton: {
    flexShrink: 0,
  },
  input: {
    flex: 1,
  },
  error: {
    color: '#d32f2f',
    fontSize: '12px',
  },
};

export default FolderPickerField;
