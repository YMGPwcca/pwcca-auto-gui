import style from '../styles/Comps.module.css'

export default function InfoCard(props: React.PropsWithChildren<{ special?: true }>) {
  return (
    <div className={`${style.info} mobile:py-4 card-bg ${props.special ? 'pc:grid-cols-3' : 'pc:grid-cols-2'} pc:grid-rows-2`}>
      {props.children}
    </div>
  )
}