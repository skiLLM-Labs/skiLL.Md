---
name: react-component-design
description: When building or refactoring React UI components to ensure reusability and maintainability.
version: 1.0.0
tags: [frontend, react, ui]
---

# React Component Design

## When to use
- Building a new UI element from scratch.
- Refactoring a large, monolithic React component.
- Extracting shared UI patterns into a component library.

## What it does
Standardizes the structure of React components by enforcing separation of concerns, strict typing, and predictable data flow, resulting in modular and testable UI code.

## Workflow
1. **Define the API**: Draft the TypeScript interface for props before writing the component.
2. **Isolate Logic**: Move complex state and side-effects into custom hooks.
3. **Build the Skeleton**: Render the static JSX structure based on props.
4. **Apply Styles**: Add CSS/Tailwind classes consistently, supporting overriding via a `className` prop.
5. **Connect Behaviors**: Bind event handlers passed from props or hooks to the JSX.

## Rules
- Components must be pure functions with respect to their props.
- File must export a strictly typed TypeScript interface for Props.
- Max 150 lines of code per component file; extract sub-components if larger.

## Anti-patterns
- **Prop Drilling**: Passing props down more than 2 levels deep (use Context or composition instead).
- **Inline Styles**: Using `style={{...}}` objects instead of a proper styling system.
- **God Components**: Handling data fetching, business logic, and UI rendering in a single component.

## Output format
A single `.tsx` file exporting one primary functional component, accompanied by a TypeScript interface named `[ComponentName]Props`.

## Example (optional)
```tsx
interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary';
  isLoading?: boolean;
}

export const Button = ({ variant = 'primary', isLoading, children, ...props }: ButtonProps) => {
  return (
    <button className={`btn-${variant}`} disabled={isLoading} {...props}>
      {isLoading ? <Spinner /> : children}
    </button>
  );
};
