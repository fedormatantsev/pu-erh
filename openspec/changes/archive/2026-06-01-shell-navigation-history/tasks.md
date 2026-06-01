## 1. Shell state: navigation history stacks

- [x] 1.1 Add `backStack: string[]` and `forwardStack: string[]` to `ShellProvider` state in `apps/desktop/src/shell.tsx`
- [x] 1.2 Update `selectBlock` to push `currentBlockId` onto `backStack` and clear `forwardStack` before setting the new id; do not record history during the initial root-id load (keep the `useEffect` calling `setCurrentBlockId` directly)
- [x] 1.3 Implement `navigateBack`: if `backStack` is non-empty, pop the last entry, push `currentBlockId` to the front of `forwardStack`, set the popped entry as `currentBlockId`
- [x] 1.4 Implement `navigateForward`: if `forwardStack` is non-empty, pop the first entry, push `currentBlockId` onto `backStack`, set the popped entry as `currentBlockId`
- [x] 1.5 Expose `navigateBack`, `navigateForward`, `canGoBack` (`backStack.length > 0`), and `canGoForward` (`forwardStack.length > 0`) in `ShellState` and the `useShell` return value

## 2. Action bar: backward and forward actions

- [x] 2.1 In `apps/desktop/src/App.tsx`, read `navigateBack`, `navigateForward`, `canGoBack`, `canGoForward` from `useShell`
- [x] 2.2 Prepend two action descriptors to `actionList` before passing to `ActionBar`: a backward action (`isDisabled: !canGoBack`, `onPress: navigateBack`) and a forward action (`isDisabled: !canGoForward`, `onPress: navigateForward`); the `id` and `label` values are to be filled in per the action bar spec — do not invent labels beyond what is minimally needed for a placeholder if the spec is silent on them
