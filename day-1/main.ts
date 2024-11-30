async function main(): Promise<void> {
  const text = await Deno.readTextFile("day-1/input.txt");
  console.log(text);
}

main()
