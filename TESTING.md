# Testing Documentation

## Test Suite Overview

The AI File Explorer includes comprehensive test coverage across all layers:

### Test Statistics
- **Total Tests**: 210+
- **Passing**: 146 (core tests)
- **Coverage**: Services, stores, and components

## Running Tests

### Unit Tests (Vitest)

```bash
# Run tests in watch mode
npm run test

# Run tests once
npm run test:run

# Generate coverage report
npm run test:coverage

# Open test UI
npm run test:ui
```

### End-to-End Tests (Playwright)

```bash
# Run E2E tests
npm run test:e2e

# Open E2E test UI
npm run test:e2e:ui

# Run tests with visible browser
npm run test:e2e:headed

# Debug tests
npm run test:e2e:debug
```

### Run All Tests

```bash
npm run test:all
```

## Test Structure

```
tests/
├── file-operations.spec.ts      # E2E tests for file operations
├── search.spec.ts               # E2E tests for search
├── global-setup.ts              # Global setup for E2E
└── global-teardown.ts           # Global teardown

src/
├── **/__tests__/                # Co-located unit tests
│   ├── FileListItem.test.ts
│   └── FileIcon.test.ts
├── components/__tests__/
├── services/__tests__/          # Service tests
│   ├── file.test.ts
│   └── query.test.ts
└── stores/__tests__/            # Store tests
    ├── files.test.ts
    ├── search.test.ts
    └── settings.test.ts
```

## Test Categories

### Unit Tests - Services

**File Service** (27 tests)
- Directory listing
- File operations (copy, move, delete)
- Permission handling
- Error scenarios

**Query Service** (26 tests)
- Query parsing
- Filter validation
- Search expression handling
- Advanced queries

**Store Tests**
- **Files Store** (30 tests): Navigation, operations, state
- **Search Store** (28 tests): Query management, results
- **Settings Store** (35 tests): Configuration, persistence

### Integration Tests

**Rust Backend**
- File system operations with real paths
- Search indexing and querying
- Permission checks
- Error handling

### E2E Tests

**File Operations**
- List files
- Display file details
- Copy files
- Move files
- Delete files

**Search**
- Search for files
- Filter results
- Clear search

## Test Configuration

### Vitest Setup (vitest.config.ts)

```typescript
export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
    },
  },
})
```

### Playwright Setup (playwright.config.ts)

```typescript
export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:5173',
    reuseExistingServer: !process.env.CI,
  },
})
```

## Test Best Practices

### Writing Tests

1. **Clear test names**
   ```typescript
   describe('FileService', () => {
     it('should list files in a directory', () => {
       // test implementation
     })
   })
   ```

2. **Setup and teardown**
   ```typescript
   beforeEach(() => {
     // setup test environment
   })
   
   afterEach(() => {
     // cleanup
   })
   ```

3. **Use factories for test data**
   ```typescript
   function createMockFile(overrides = {}) {
     return { name: 'test.txt', size: 1024, ...overrides }
   }
   ```

4. **Test behavior, not implementation**
   ```typescript
   // Good: Tests the public API
   expect(service.listFiles(path)).toHaveLength(3)
   
   // Bad: Tests internals
   expect(service.cache.get(path)).toBeDefined()
   ```

### Component Testing

```typescript
import { mount } from '@vue/test-utils'
import FileListItem from '@/components/FileExplorer/FileListItem.vue'

describe('FileListItem', () => {
  it('emits select event on click', async () => {
    const file = { name: 'test.txt', path: '/test.txt' }
    const wrapper = mount(FileListItem, {
      props: { file, selected: false },
    })
    
    await wrapper.find('.file-item').trigger('click')
    expect(wrapper.emitted('select')).toBeTruthy()
  })
})
```

## Continuous Integration

### GitHub Actions Setup

```yaml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: npm ci
      - run: npm run test:run
      - run: npm run test:e2e
```

## Test Debugging

### Debug Mode

```bash
# Debug E2E tests
npm run test:e2e:debug

# Debug unit tests
node --inspect-brk ./node_modules/.bin/vitest
```

### Visual Debugging

```typescript
// In Playwright tests
test('debug this test', async ({ page }) => {
  await page.pause() // Opens Inspector
  // Test continues after resuming
})
```

## Performance Testing

### Load Testing

```typescript
it('should handle 10,000 files', async () => {
  const files = Array.from({ length: 10000 }, (_, i) => ({
    name: `file${i}.txt`,
    path: `/test/file${i}.txt`,
  }))
  
  const start = performance.now()
  const sorted = sortFiles(files)
  const duration = performance.now() - start
  
  expect(duration).toBeLessThan(100) // Must be fast
})
```

## Known Test Issues

### Environment-related
1. **DOM events in test environment** - Some Vue Test Utils limitations
2. **CSS testing** - Styling verification is limited
3. **File system mocking** - Real operations in integration tests

### Workarounds
- Use happy-dom for faster testing
- Mock file operations where needed
- Focus on behavior testing, not implementation

## Test Maintenance

### Updating Tests
1. Change functionality first
2. See tests fail
3. Update tests to match new behavior
4. Verify all tests pass

### Debugging Flaky Tests
```bash
# Increase timeout for slow operations
it('slow operation', async () => {
  // test code
}, { timeout: 10000 })
```

## Coverage Goals

- **Statements**: >90%
- **Branches**: >85%
- **Functions**: >90%
- **Lines**: >90%

## Resources

- [Vitest Documentation](https://vitest.dev)
- [Playwright Documentation](https://playwright.dev)
- [Vue Test Utils](https://test-utils.vuejs.org)
- [Testing Library Best Practices](https://testing-library.com/docs)
