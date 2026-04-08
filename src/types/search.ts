export interface SearchQuery {
  text: string
  filters?: SearchFilters
  limit?: number
  offset?: number
}

export interface SearchFilters {
  type?: string[]
  minSize?: number
  maxSize?: number
  beforeDate?: Date
  afterDate?: Date
  tags?: string[]
  path?: string
}

export interface SearchResult {
  file: any
  score: number
  highlights?: string[]
  reason?: string
}

export interface SearchState {
  query: string
  results: SearchResult[]
  isSearching: boolean
  error?: string
}
