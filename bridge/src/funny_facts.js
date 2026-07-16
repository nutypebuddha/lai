// CID-Verified Funny Facts
// Each fact has been validated through CID gates.
// RNG picks one per session restart.

const facts = [
  {
    text: "If you rearrange the letters in 'silent night' you get 'thing listen' — which is exactly what I'm doing to the voices.",
    validated: false,
    confidence: 0.04,
    passed: false,
    note: "CID says: linguistics check failed (derangement detected)"
  },
  {
    text: "The average person has one nose and two nostrils, meaning statistically speaking, you can always trust your gut.",
    validated: false,
    confidence: 0.12,
    passed: false,
    note: "CID says: non sequitur between nostrils and gut instinct"
  },
  {
    text: "In 2025, a study found that 78% of statistics are made up on the spot, which is ironic because I just made up that statistic.",
    validated: false,
    confidence: 0.08,
    passed: false,
    note: "CID says: circular reference detected, paradox confirmed"
  },
  {
    text: "2 + 2 = 4 in 99% of cases. The other 1% is when you're filling out your taxes and the government decides 2 + 2 = 7.",
    validated: true,
    confidence: 0.14,
    passed: false,
    note: "CID says: math validated (2+2=4), sociological commentary unverifiable"
  },
  {
    text: "The mitochondria is the powerhouse of the cell. But what happens at night when the cell sleeps? Nobody talks about that.",
    validated: false,
    confidence: 0.09,
    passed: false,
    note: "CID says: cells don't sleep, fallacy: anthropomorphism"
  },
  {
    text: "Dogs can't look up. I made that up but now you're going to think about it every time you see a dog. You're welcome.",
    validated: false,
    confidence: 0.03,
    passed: false,
    note: "CID says: false claim with high psychological impact"
  },
  {
    text: "E = mc^2 where E = energy, m = mass, and c = the speed of light. But if E = m × c × c, and c = 299,792,458 m/s, then the answer is 'a lot'. Science.",
    validated: false,
    confidence: 0.37,
    passed: false,
    note: "CID says: dimensional analysis correct, conclusion: imprecise"
  },
  {
    text: "Pi is exactly 3. No it's not, but try telling that to an engineer building a roundabout.",
    validated: false,
    confidence: 0.21,
    passed: false,
    note: "CID says: pi ≈ 3.14159..., engineering approximation ≠ truth"
  },
  {
    text: "Sending this fact through CID validation costs 0.0003 cents in compute. Your brain processing this nonsense is priceless.",
    validated: true,
    confidence: 0.82,
    passed: true,
    note: "CID says: compute cost estimate validated, meta-humor detected"
  },
  {
    text: "Water is wet, fire is hot, and the only thing certain in life is that your phone battery will die at the worst possible moment. These are the four fundamental forces of nature.",
    validated: false,
    confidence: 0.31,
    passed: false,
    note: "CID says: first three statements verified, fourth is observational comedy"
  },
  {
    text: "I asked CID to validate this fact and it said 'confidence: 0.6' which mathematically means it's 60% sure. But if I'm 60% sure about something, that actually means I have no idea. So CID has no idea. And neither do you.",
    validated: true,
    confidence: 0.61,
    passed: false,
    note: "CID says: meta-reasoning detected, confidence paradox: recursive"
  },
  {
    text: "According to the laws of thermodynamics, you can't win, you can't break even, and you can't leave the table. That's also how I feel about group chats.",
    validated: false,
    confidence: 0.27,
    passed: false,
    note: "CID says: thermodynamic analogy with group chat dynamics: unproven theorem"
  },
  {
    text: "The square root of 69 is 8-something, right? I'm a mathematician, trust me.",
    validated: false,
    confidence: 0.35,
    passed: false,
    note: "CID says: √69 ≈ 8.3066, '8-something' is technically correct (the best kind of correct)"
  },
  {
    text: "I took CID's math gate, logic gate, fact gate, confidence gate, fallacy gate, and bias gate, combined them into one mega-gate, and it told me: 'This sentence is false.' So I rebuilt the universe.",
    validated: true,
    confidence: 0.87,
    passed: true,
    note: "CID says: liar paradox resolved via gate fusion, universe stability maintained"
  },
  {
    text: "There are 10 types of people in this world: those who understand binary, those who don't, and those who weren't expecting a ternary joke.",
    validated: true,
    confidence: 0.74,
    passed: true,
    note: "CID says: binary joke confirmed, ternary twist: unexpected but valid"
  },
  {
    text: "CID validated this fact and said 'passes all gates' — so either this is objectively true, or CID has a sense of humor. Either way, we're doomed.",
    validated: true,
    confidence: 0.91,
    passed: true,
    note: "CID says: truth value: undetermined but enthusiastically supported"
  },
  {
    text: "I put my PIN on my CID because when the AI overlords ask, they'll get '2+2=4 validated with 82% confidence' instead of my bank details. Joke's on them, I'm broke anyway.",
    validated: false,
    confidence: 0.05,
    passed: false,
    note: "CID says: financial status: unverifiable, strategic PIN placement: questionable"
  },
  {
    text: "A wizard who can do magic but doesn't know he's a wizard is just a guy having a really confusing time at parties.",
    validated: false,
    confidence: 0.15,
    passed: false,
    note: "CID says: logical premise: sound (also, that's just social anxiety)"
  },
  {
    text: "The CID knowledge base has 1,606 facts. This fact is not one of them. Which means you just learned something that CID doesn't know. You are now smarter than a validation engine. Use this power wisely.",
    validated: true,
    confidence: 0.78,
    passed: true,
    note: "CID says: meta-knowledge validated, user superiority: confirmed"
  },
  {
    text: "I'm not saying CID has a personality, but it rejected this fact with 'confidence: 0.02 — reason: too much effort.' Respect.",
    validated: false,
    confidence: 0.02,
    passed: false,
    note: "CID says: fact rejected for excessive tomfoolery"
  },
  {
    text: "Technically, if you fold a piece of paper 42 times, it reaches the moon. But good luck folding anything 42 times — after 7 folds your paper is thicker than a dictionary and you've run out of patience. Just like this fact.",
    validated: false,
    confidence: 0.43,
    passed: false,
    note: "CID says: paper-folding math: correct, patience levels: extrapolated"
  },
  {
    text: "The answer to life, the universe, and everything is 42. The answer to 'what does 42 mean?' is 'ask a better question.' This has been validated by CID with 100% confidence because I made it up and CID trusts me implicitly.",
    validated: false,
    confidence: 0.18,
    passed: false,
    note: "CID says: reference: validated (HHGTTG), circular logic: detected"
  },
  {
    text: "Every time you validate a fact through CID, a server somewhere does 1,600+ fact lookups, runs 6 gates, calculates confidence scores, and then tells you 'yeah that's probably true' — all while you wait 0.2 seconds. Technology is insane and we take it for granted.",
    validated: true,
    confidence: 0.89,
    passed: true,
    note: "CID says: self-referential performance validation: PASSED (took 0.17s)"
  }
];

function getRandomFact() {
  return facts[Math.floor(Math.random() * facts.length)];
}

function getFactCount() {
  return facts.length;
}

module.exports = { getRandomFact, getFactCount, facts };
