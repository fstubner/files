export interface FileItem {
  id: string
  path: string
  name: string
  type: 'file' | 'directory' | 'symlink' | 'cloud'
  size: number
  modified: Date
  created: Date
  permissions: FilePermissions
  isHidden: boolean
  isReadOnly: boolean
  mimeType?: string
  extension?: string
  tags: string[]
  metadata: FileMetadata
  cloudProvider?: CloudProvider
}

export interface FilePermissions {
  owner: string
  group: string
  mode: number
  readable: boolean
  writable: boolean
  executable: boolean
}

export interface FileMetadata {
  width?: number
  height?: number
  duration?: number
  bitrate?: number
  codec?: string
  pages?: number
  language?: string
  aiGenerated?: boolean
  confidence?: number
}

export interface CloudProvider {
  name: 'google-drive' | 'dropbox' | 'onedrive' | 'icloud' | 'box' | 's3'
  id: string
  path: string
  syncStatus: 'synced' | 'syncing' | 'conflict' | 'error'
}

export interface FileStats {
  totalFiles: number
  totalDirs: number
  totalSize: number
  averageFileSize: number
  largestFile: FileItem | null
}
