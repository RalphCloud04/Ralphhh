# Ralphhh
PROJECT NAME: CropPay
⦁	PROBLEM (1 sentence): A rice farmer in Nueva Ecija struggles to get paid fairly and quickly after harvest, often waiting weeks and losing income to middlemen.
⦁	SOLUTION (1 sentence): A Soroban smart contract on Stellar lets buyers send USDC directly to farmers’ wallets, with instant settlement and transparent records.
⦁	STELLAR FEATURES USED: [X] USDC transfer [X] Soroban smart contract [X] Trustline
⦁	TARGET USERS: Smallholder rice farmers in rural Philippines and urban rice distributors who want faster, cheaper, and traceable payments.
⦁	CORE FEATURE (MVP): A buyer sends USDC to a farmer’s wallet, and the contract logs the payment as “verified harvest payment.”
⦁	CONSTRAINTS:
⦁	Region: SEA
⦁	User type: Farmers, SMEs
⦁	Complexity: Mobile-first
⦁	Theme: Agriculture & Supply Chain → Farmer payments

Description: This code defines a Soroban smart contract called CropPay that allows buyers to pay farmers directly in USDC and records those payments in persistent storage. The pay function requires buyer authorization and logs the payment, while the check function lets anyone verify if a payment between a buyer and farmer exists. The included tests confirm correct payment recording, enforce authorization, and validate stored state.
STELLAR LINK: https://stellar.expert/explorer/testnet/tx/8d99339f4c8d1337d6a5bf994a4e565f58ced4bc0635b99862a0227b3b81ab50
Contract ID: 8d99339f4c8d1337d6a5bf994a4e565f58ced4bc0635b99862a0227b3b81ab50