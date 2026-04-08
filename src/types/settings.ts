export interface AppSettings {
  theme: 'light' | 'dark' | 'system'
  fontSize: number
  defaultPath: string
  language: string
  autoplay: boolean
  showHiddenFiles: boolean
  sortBy: 'name' | 'date' | 'size' | 'type'
  sortOrder: 'asc' | 'desc'
  enableAI: boolean
  enableCloudSync: boolean
  cloudProviders?: CloudSettings[]
}

export interface CloudSettings {
  provider: string
  token?: string
  refreshToken?: string
  expiresAt?: number
}
