export default function InfoCard(props: React.PropsWithChildren<{ special?: true }>) {
  return (
    <div className={`text-white rounded-xl h-72 w-72 justify-center font-bold grid grid-cols-2 grid-rows-2 card-bg gap-4 p-4`}>
      {props.children}
    </div>
  )

}