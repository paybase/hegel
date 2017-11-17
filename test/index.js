const iterations = parseInt(process.argv[ 2 ], 10);
console.log(process.env.FROM_HEGEL);

const timeout = ms => new Promise(resolve => setTimeout(resolve, ms));

(async () => {
  console.error('checking stderr');
  for (let i = 0; i < iterations; i++) {
    console.log(i);
    await timeout(1000);
  }
  process.exit();
})();
