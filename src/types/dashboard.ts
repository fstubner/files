export interface DashboardWidget {
  id: string
  name: string
  type: string
  position: number
  size: 'small' | 'medium' | 'large'
  config?: Record<string, any>
}

export interface DashboardStats {
  totalFiles: number
  totalDirs: number
  totalSize: number
  recentFiles: any[]
  topTags: Array<{ name: string; count: number }>
}
