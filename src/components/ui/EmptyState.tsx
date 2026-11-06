interface EmptyStateProps {
  title: string;
  description: string;
  children?: React.ReactNode;
}

export default function EmptyState({
  title,
  description,
  children,
}: EmptyStateProps) {
  return (
    <section style={{ padding: "64px 32px" }}>
      <h1 style={{ fontSize: 24, fontWeight: 600, margin: 0 }}>{title}</h1>
      <p style={{ color: "var(--muted)", marginTop: 8 }}>{description}</p>
      {children}
    </section>
  );
}
