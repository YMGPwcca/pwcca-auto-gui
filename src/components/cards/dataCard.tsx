import style from '../styles/Comps.module.css'

export default function DataCard(props: React.ComponentProps<'div'>) {
  const withoutClassName = (({ className, ...others }) => others)(props)

  return (
    <div {...withoutClassName} className={`${style.data} info-bg ${props.className || ''}`}>
      {props.children}
    </div>
  )
}