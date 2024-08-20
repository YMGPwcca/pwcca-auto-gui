export default function ItemCard(props: React.PropsWithChildren<{ special?: true }>) {
  return (
    <div id='ItemCard' className='transition-all duration-75 text-white rounded-xl h-72 w-72 justify-center font-bold grid grid-cols-2 grid-rows-2 bg-tier1 gap-4 p-4'>
      {props.children}
    </div>
  )

}