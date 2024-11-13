let pillsRemaining: number[] = [];
let winningPillNumber: number;
let potentialWinnings = 0;
let pillsConsumed = 0;

function initializePills() {
  pillsRemaining = Array.from({length: 1000}, (_, i) => i + 1);
  winningPillNumber = Math.floor(Math.random() * 1000) + 1;
  pillsConsumed = 0;
}

document.getElementById('lucky-button')?.addEventListener('click', () => {
  const luckyButton = document.getElementById('lucky-button') as HTMLButtonElement;
  const pillCount = parseInt((document.getElementById('pill-count') as HTMLInputElement).value) || 1;
  let selectedPills: number[] = [];
    
  // Randomly select pills from remaining pills
  for (let i = 0; i < pillCount; i++) {
      if (pillsRemaining.length === 0) break;
      const randomIndex = Math.floor(Math.random() * pillsRemaining.length);
      selectedPills.push(pillsRemaining.splice(randomIndex, 1)[0]);
      pillsConsumed += 1;
  }
    
  const resultDiv = document.getElementById('result');
  if (selectedPills.includes(winningPillNumber)) {
      resultDiv!.textContent = `that's what happens when you eat ${pillsConsumed} pills`;
      luckyButton.disabled = true;
      luckyButton.style.opacity = '0.5';
  } else {
      potentialWinnings += selectedPills.length * 50000;
      resultDiv!.textContent = "You greedy bastard. You want more pills?";
      document.getElementById('current-odds')!.textContent = 
          `Pills remaining: ${pillsRemaining.length}\n` +
          `Potential Prize: $${potentialWinnings.toLocaleString()}`;
  }
});

document.getElementById('reset-button')?.addEventListener('click', () => {
  const luckyButton = document.getElementById('lucky-button') as HTMLButtonElement;
  luckyButton.disabled = false;
  luckyButton.style.opacity = '1';
  initializePills();
  potentialWinnings = 0;
  document.getElementById('current-odds')!.textContent = 
      `Pills remaining: 1000\n` +
      `Potential Prize: $0`;
  document.getElementById('result')!.textContent = '';
});

// Initialize on page load
initializePills();
