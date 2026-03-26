import { useState } from 'react';
import type { CSSProperties } from 'react';
import { Button, Input, Label, Text } from '@fluentui/react-components';
import { FolderPickerField } from './FolderPickerField';
import { ScheduleSelect } from './ScheduleSelect';
import { FeedbackMessage } from '../../../components/ui';

interface SyncProfileFormData {
  name: string;
  sourcePath: string;
  destinationPath: string;
  schedule: string;
  enabled: boolean;
}

interface SyncProfileFormProps {
  initialData?: Partial<SyncProfileFormData>;
  onSubmit: (data: SyncProfileFormData) => void;
  onCancel?: () => void;
  isSubmitting?: boolean;
  error?: string;
  success?: string;
}

export function SyncProfileForm({
  initialData,
  onSubmit,
  onCancel,
  isSubmitting = false,
  error,
  success,
}: SyncProfileFormProps) {
  const [formData, setFormData] = useState<SyncProfileFormData>({
    name: initialData?.name || '',
    sourcePath: initialData?.sourcePath || '',
    destinationPath: initialData?.destinationPath || '',
    schedule: initialData?.schedule || '60',
    enabled: initialData?.enabled ?? true,
  });

  const [errors, setErrors] = useState<Partial<Record<keyof SyncProfileFormData, string>>>({});

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof SyncProfileFormData, string>> = {};
    
    if (!formData.name.trim()) {
      newErrors.name = 'Vui lòng nhập tên hồ sơ';
    }
    if (!formData.sourcePath.trim()) {
      newErrors.sourcePath = 'Vui lòng chọn thư mục nguồn';
    }
    if (!formData.destinationPath.trim()) {
      newErrors.destinationPath = 'Vui lòng chọn thư mục đích';
    }
    if (formData.sourcePath === formData.destinationPath && formData.sourcePath) {
      newErrors.destinationPath = 'Thư mục đích phải khác thư mục nguồn';
    }
    
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validate()) {
      onSubmit(formData);
    }
  };

  return (
    <form 
      onSubmit={handleSubmit}
      style={styles.form}
      aria-label="Form tạo hồ sơ đồng bộ"
    >
      <h2 style={styles.title}>
        {initialData?.name ? 'Chỉnh sửa hồ sơ' : 'Tạo hồ sơ đồng bộ mới'}
      </h2>

      {error && (
        <FeedbackMessage
          type="error"
          title="Lỗi"
          description={error}
        />
      )}

      {success && (
        <FeedbackMessage
          type="success"
          title="Thành công"
          description={success}
        />
      )}

      <div style={styles.field}>
        <Label htmlFor="profile-name" required>
          Tên hồ sơ
        </Label>
        <Input
          id="profile-name"
          value={formData.name}
          onChange={(_, data) => setFormData(prev => ({ ...prev, name: data.value }))}
          placeholder="VD: Đồng bộ tài liệu"
          disabled={isSubmitting}
          aria-describedby={errors.name ? 'profile-name-error' : undefined}
          aria-invalid={!!errors.name}
        />
        {errors.name && (
          <Text id="profile-name-error" role="alert" style={styles.error}>
            {errors.name}
          </Text>
        )}
      </div>

      <FolderPickerField
        id="source"
        label="Thư mục nguồn"
        value={formData.sourcePath}
        onPathChange={(path) => setFormData(prev => ({ ...prev, sourcePath: path }))}
        disabled={isSubmitting}
        error={errors.sourcePath}
        required
      />

      <FolderPickerField
        id="destination"
        label="Thư mục đích"
        value={formData.destinationPath}
        onPathChange={(path) => setFormData(prev => ({ ...prev, destinationPath: path }))}
        disabled={isSubmitting}
        error={errors.destinationPath}
        required
      />

      <ScheduleSelect
        id="schedule"
        label="Lịch đồng bộ"
        value={formData.schedule}
        onChange={(value) => setFormData(prev => ({ ...prev, schedule: value }))}
        disabled={isSubmitting}
        required
      />

      <div style={styles.actions}>
        <Button
          type="submit"
          appearance="primary"
          disabled={isSubmitting}
          aria-label="Lưu hồ sơ đồng bộ"
        >
          {isSubmitting ? 'Đang lưu...' : 'Lưu hồ sơ'}
        </Button>
        {onCancel && (
          <Button
            type="button"
            onClick={onCancel}
            disabled={isSubmitting}
            aria-label="Hủy bỏ"
          >
            Hủy
          </Button>
        )}
      </div>
    </form>
  );
}

const styles: Record<string, CSSProperties> = {
  form: {
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
    maxWidth: '500px',
    padding: '24px',
    backgroundColor: '#ffffff',
    borderRadius: '8px',
  },
  title: {
    marginBottom: '8px',
    fontSize: '24px',
    fontWeight: 600,
    color: '#1f1f1f',
  },
  field: {
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
  },
  error: {
    color: '#d32f2f',
    fontSize: '12px',
  },
  actions: {
    display: 'flex',
    gap: '12px',
    marginTop: '8px',
  },
};

export default SyncProfileForm;
