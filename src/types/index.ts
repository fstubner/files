// Export all types for easy importing
export * from './fileSystem'
export * from './tags'
export * from './search'
export * from './dashboard'
export * from './contextMenu'
export * from './ai'
export * from './events'
export * from './settings'
export * from './tabs'

// Common utility types
export type Nullable<T> = T | null
export type Optional<T> = T | undefined
export type Awaitable<T> = T | Promise<T>
