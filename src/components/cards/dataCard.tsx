export default function DataCard(props: React.ComponentProps<'div'>) {
  const withoutClassName = (({ className, ...others }) => others)(props)

  return (
    <div {...withoutClassName} className={`flex flex-col text-center justify-center rounded-lg bg-tier2 ${props.className || ''}`}>
      {props.children}
    </div>
  )
}