// LogDetailView component (Story 3.4 AC#1-4)
// Displays detailed log entries for a specific job with human-readable error messages
import React, { useState, useEffect } from 'react';
import { Text, Spinner, MessageBar, MessageBarBody } from '@fluentui/react-components';
import { CheckmarkCircle24Regular, ErrorCircle24Regular, Warning24Regular, Info24Regular } from '@fluentui/react-icons';
import { getJobLogs } from '../../../services/tauri/commands';
import type { SyncEventLog, LogEntry } from '../../../types/log';
import { mapErrorToUserMessage, getSuggestionForError, getErrorGroup } from './errorMappings';

interface LogDetailViewProps {
  jobId: string;
}

export const LogDetailView: React.FC<LogDetailViewProps> = ({ jobId }) => {
  const [logs, setLogs] = useState<SyncEventLog[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadJobLogs();
  }, [jobId]);

  const loadJobLogs = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const result = await getJobLogs(jobId);
      
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? 'Failed to load logs');
      }
      
      setLogs(result.data);
    } catch (err) {
      console.error('[LogDetailView] Load error:', err);
      setError('Không thể tải chi tiết log');
    } finally {
      setIsLoading(false);
    }
  };

  // Loading state
  if (isLoading) {
    return (
      <div style={{ 
        display: 'flex', 
        flexDirection: 'column', 
        alignItems: 'center', 
        justifyContent: 'center',
        height: '100%',
        gap: '16px'
      }}>
        <Spinner size="large" label="Đang tải chi tiết..." />
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <MessageBar intent="error">
        <MessageBarBody>{error}</MessageBarBody>
      </MessageBar>
    );
  }

  // Convert to display entries
  const displayLogs: LogEntry[] = logs.map(log => ({
    id: log.id,
    timestamp: log.timestamp,
    level: log.eventType === 'job_failed' || log.errorCode ? 'error' : 
           log.eventType === 'job_completed' ? 'success' :
           log.eventType === 'job_progress' ? 'info' : 'info',
    message: log.errorCode ? mapErrorToUserMessage(log.errorCode) : log.message,
    details: log.details,
    errorCode: log.errorCode,
  }));

  return (
    <div>
      <Text size={400} weight="semibold" style={{ marginBottom: '16px', display: 'block' }}>
        Chi tiết log
      </Text>

      {/* Error summary if job failed (Story 3.4 AC#2) */}
      {logs.some(l => l.eventType === 'job_failed') && (
        <ErrorSummary logs={logs} />
      )}

      {/* Log entries */}
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        {displayLogs.map((log) => {
          const LevelIcon = log.level === 'success' ? CheckmarkCircle24Regular :
                           log.level === 'error' ? ErrorCircle24Regular :
                           log.level === 'warning' ? Warning24Regular : Info24Regular;
          
          const iconColor = log.level === 'success' ? '#4CAF50' :
                           log.level === 'error' ? '#D32F2F' :
                           log.level === 'warning' ? '#FFC107' : '#0078D4';
          
          return (
            <div 
              key={log.id}
              style={{
                padding: '12px',
                backgroundColor: log.level === 'error' ? '#FEF2F2' : 
                                 log.level === 'warning' ? '#FFF8E1' : '#F3F3F3',
                borderRadius: '4px',
                borderLeft: `3px solid ${iconColor}`
              }}
            >
              <div style={{ display: 'flex', alignItems: 'flex-start', gap: '8px' }}>
                <LevelIcon style={{ color: iconColor, flexShrink: 0, marginTop: '2px' }} />
                <div style={{ flex: 1 }}>
                  <Text size={200} weight="medium">{log.message}</Text>
                  <Text size={200} style={{ display: 'block', color: '#616161', marginTop: '4px' }}>
                    {formatTime(log.timestamp)}
                  </Text>
                  {log.details && (
                    <Text size={200} style={{ display: 'block', marginTop: '8px' }}>
                      {log.details}
                    </Text>
                  )}
                  {log.errorCode && (
                    <Text size={200} style={{ display: 'block', marginTop: '8px', color: '#D32F2F' }}>
                      Mã lỗi: {log.errorCode}
                    </Text>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};

// Error summary component with actionable suggestions (Story 3.4 AC#2)
const ErrorSummary: React.FC<{ logs: SyncEventLog[] }> = ({ logs }) => {
  const errorLog = logs.find(l => l.errorCode);
  if (!errorLog || !errorLog.errorCode) return null;
  
  const userMessage = mapErrorToUserMessage(errorLog.errorCode);
  const suggestion = getSuggestionForError(errorLog.errorCode);
  const errorGroup = getErrorGroup(errorLog.errorCode);
  
  const groupLabels: Record<string, string> = {
    availability: '🔌 Kết nối',
    access: '🔐 Quyền truy cập',
    lock: '📁 File đang bị khóa',
    validation: '⚙️ Cấu hình',
  };
  
  return (
    <MessageBar intent="error" style={{ marginBottom: '16px' }}>
      <MessageBarBody>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '4px' }}>
          <Text weight="semibold">{userMessage}</Text>
          {errorGroup && (
            <Text size={200} style={{ 
              padding: '2px 8px', 
              backgroundColor: '#FEF2F2', 
              borderRadius: '4px',
              border: '1px solid #D32F2F'
            }}>
              {groupLabels[errorGroup] || errorGroup}
            </Text>
          )}
        </div>
        {suggestion && (
          <Text size={200} style={{ display: 'block', marginTop: '8px' }}>
            💡 {suggestion}
          </Text>
        )}
      </MessageBarBody>
    </MessageBar>
  );
};

function formatTime(timestamp: string): string {
  const date = new Date(timestamp);
  return date.toLocaleTimeString('vi-VN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}

export default LogDetailView;
