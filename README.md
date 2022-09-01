# Cipherbox
Safely keep data private on public distributed storage like Filecoin/IPFS

###Motivation

Data saved on a single device may be lost for various reasons, such as improper operation, hardware failures, and natural disasters. One of the main methods to protect important data is backup. The widely used decentralized storage networks, such as IPFS & Filecoin, is ideal for data backup. It currently hosts large amount of open public data. Bear in mind that private data should be encryted before put it to public networks.  But the whole process that users facing would be daunting. CipherBox aim to ease the backup process and help users mange backup tasks in a relaxed and enjoyed way.


###Design

*Security*

User data will be encryted before transfering to public distributed stroage network. So the main security issue is how to keep password as safe as possible. Save passwords on server side is definitely not an option, we can't assume that third party will be always trustful. So can we saving passwords on user's devices? No, there are risks of being compromised by hackers. 

The safest place to save password is where only the password's owner knonws, and the outside world has zero-knowledge about the password. 

Cipherbox do not save user password in any place. During the password setup when first time open cipherbox, it will derived a secret key for future encrypting/decrypting base on user inputted password and an random generated nonce. Then encrypts one pre-defined message by the secret key, and save the encrypted message and the nonce to user device. In this way, cipherbox only know how to derived a secret key from password and how to do varification.