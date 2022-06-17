import init from "/js/ch_E_ap.js";

let encodeLength, decodeLength;
let encodeBlob, encodeString, encodePassphrase;
let jsRead, jsWrite;

const rustWasm = await init();
const sharedBufferSize = rustWasm.get_WASM_MEMORY_BUFFER_SIZE();
let wasmPointer = null;

// https://stackoverflow.com/questions/25354313/saving-a-uint8array-to-a-binary-file

/*
function openFile(){
    var file = $('#file-selector').prop('files')[0];

    var fileReader = new FileReader();
    fileReader.onloaded = function (){
          var blob = fileReader.result;
          doSomethin(blob);
    };
    fileReader.readAsArrayBuffer(file);
}*/	

/***********************************************
                  COMMON
 **********************************************/

encodeBlob = function(payload) {
  var len = payload.byteLength;
  var head = encodeLength(len);
  if ( head === null ) return null;

  var result = new Uint8Array(len+2);
  result.set(head,0)
  result.set(payload,2)
  return result;
}

encodeString = function(str) {
  return encodeBlob(new TextEncoder().encode(str))
}

//read, clean, encode passphrase
encodePassphrase = function() {
  var txtarea = document.getElementById("passphrase");
  var passphrase = txtarea.value;
  txtarea.value = "";
  
  var payload = encodeString(passphrase);
  if (payload === null) return new Uint8Array([0,0]);

  return payload;
}


encodeLength = function(len) {
  return (len > sharedBufferSize-2) ? null : [Math.floor(len/256),Math.floor(len%256)];
}

decodeLength = function(buffer) {
  return 256*buffer[0] + buffer[1];
}

jsRead = function(buffer,offset){
  buffer = buffer.subarray(offset);

  let length = decodeLength(buffer);
  offset += 2;
  
  let data = buffer.subarray(2,2+length);
  offset += length;

  return [data,offset];
}

jsWrite = function(buffer,offset,data) {  
  let length = decodeLength(data);
  if (offset+length+2 > sharedBufferSize) {
    alert("Trying to pass too much data.")
    return offset;
  }
  else {
    buffer.set(data,offset);
    return offset+length+2;
  }
}

/***********************************************
                  END
 **********************************************/



/***********************************************
                  GENERATE
 **********************************************/
let downloadBlob, downloadURL, formatForFile;
let defineWasmGenFun, retrieveKeys;
                  
async function generate() {
  // Defining which algorithm to use
  const wasm_generate_keypair = defineWasmGenFun(rustWasm);
  if (wasm_generate_keypair === null) return;
                    
  // Instead of generate a new buffer each time, clean it after use
  if (wasmPointer === null) wasmPointer = rustWasm.wasm_new_buffer();
                    
  // Write in memory the passphrase submited (if there was any)
  let wasmBuffer = new Uint8Array(rustWasm.memory.buffer,wasmPointer,sharedBufferSize);
  let passphrase = encodePassphrase();
                    
  // Time to work with wasm module
  let toClean = jsWrite(wasmBuffer,0,passphrase);
                      
  wasm_generate_keypair(wasmPointer);
                    
  // Need to renew wasmBuffer after linear memory has been altered
  wasmBuffer = new Uint8Array(rustWasm.memory.buffer,wasmPointer,sharedBufferSize);
  let keys = retrieveKeys(wasmBuffer);
  toClean = Math.max(toClean,keys[2]);
                      
  for (const u8s of formatForFile(keys)) {
    downloadBlob(u8s.value, `${u8s.name}.${u8s.extension}`, u8s.mime);
  }
  rustWasm.wasm_clean_buffer(wasmPointer,toClean);
};
                    
downloadBlob = function(data, fileName, mimeType) {
  var blob, url;
  blob = new Blob([data], {
    type: mimeType
  });
  url = window.URL.createObjectURL(blob);
  downloadURL(url, fileName);
  setTimeout(function() {
    return window.URL.revokeObjectURL(url);
  }, 1000);
};
                    
downloadURL = function(data, fileName) {
  var a;
  a = document.createElement('a');
  a.href = data;
  a.download = fileName;
  document.body.appendChild(a);
  a.style = 'display: none';
  a.click();
  a.remove();
};
                    
defineWasmGenFun = function(module) {
  var algo = document.getElementById("algo").value;
  if ( algo === 'RSA') return           module.wasm_generate_keypair_rsa;
  else if ( algo === 'ECDSA' ) return   module.wasm_generate_keypair_ecdsa;
  else if ( algo === 'Ed25519' ) return module.wasm_generate_keypair_ed25519;
  else return null;
}
                    
retrieveKeys = function(buffer) {
  let offset = 0;
                    
  let publicKey = jsRead(buffer,offset);
  offset = publicKey[1];
                    
  let privateKey = jsRead(buffer,offset); 
  offset = privateKey[1];
                    
  return [publicKey[0],privateKey[0],offset];
}
                    
formatForFile = function(keys) {
  return new Array(
    { name: "private_key", extension: "priv", mime: 'text/pem', value: keys[1] },
    { name: "public_key", extension: "pub", mime: 'text/openssh', value: keys[0] }
  );
}
                    

/***********************************************
                  END
**********************************************/

/***********************************************
                  SIGN
 **********************************************/
async function sign(data) {
	let file = $('#private-file').prop('files')[0];
  
  let fileContent = await file.arrayBuffer();
  fileContent = new Uint8Array(fileContent, 0, fileContent.byteLength);

  data = encodeString(data);
  fileContent = encodeBlob(fileContent);
  const passphrase = encodePassphrase();

  /************* Write *************/

  let offset = 0;
  let wasmBuffer = new Uint8Array(rustWasm.memory.buffer,wasmPointer,sharedBufferSize);

  offset = jsWrite(wasmBuffer,offset,data);
  wasmBuffer = new Uint8Array(rustWasm.memory.buffer,wasmPointer,sharedBufferSize);
  offset = jsWrite(wasmBuffer,offset,fileContent);
  wasmBuffer = new Uint8Array(rustWasm.memory.buffer,wasmPointer,sharedBufferSize);
  offset = jsWrite(wasmBuffer,offset,passphrase);

  console.log(wasmBuffer);
  /************* Read *************/
  
  await rustWasm.wasm_sign(wasmBuffer);
  
  offset = 0;
  wasmBuffer = new Uint8Array(rustWasm.memory.buffer,wasmPointer,sharedBufferSize);

  const signature_raw = jsRead(wasmBuffer,offset)[0];
  const signature = new TextDecoder().decode(signature_raw);

  /* Trace it */
  console.log(signature);
  const elem = document.getElementById('signature');
  if (elem != null) elem.value = signature;

  return signature;
};


function test_sign() {
  sign(document.getElementById('data').value)
}


/***********************************************
                  END
**********************************************/

/***********************************************
                VERIFY
 **********************************************/

async function verify() {
	var file = $('#public-file').prop('files')[0];
  
  let fileContent = await file.arrayBuffer();
  fileContent = new Uint8Array(fileContent, 0, fileContent.byteLength);

  data = encodeString(document.getElementById('data').value);
  const signature = encodeString(document.getElementById('signature').value);
  fileContent = encodeBlob(fileContent);

  /************* Write *************/

  let offset = 0;
  let wasmBuffer = new Uint8Array(rustWasm.memory.buffer,wasmPointer,sharedBufferSize);

  offset = jsWrite(wasmBuffer,offset,data);
  wasmBuffer = new Uint8Array(rustWasm.memory.buffer,wasmPointer,sharedBufferSize);
  offset = jsWrite(wasmBuffer,offset,signature);
  wasmBuffer = new Uint8Array(rustWasm.memory.buffer,wasmPointer,sharedBufferSize);
  offset = jsWrite(wasmBuffer,offset,fileContent);

  console.log(wasmBuffer);
  
  /************* Read *************/
  
  if (rustWasm.wasm_verify(wasmBuffer)) alert("Signature verification succeed")
  else alert("Signature verification failed")
  
}

/***********************************************
                  END
**********************************************/

/***********************************************
                  AUTH
**********************************************/

function authentificate() {
  let user = document.getElementById('user.name').value;
    if (user != '') stg1(user);
}
  
function stg1(user) {
  let payload = 'stage=1&user='+ user +'&signed=';
  $.post("/login",payload,"text")
    // Clean it later
    .always( () => { console.log("1: "+payload); })
    .done(
      resp => {
        stg2(user,resp);
      }
    )            
    .fail( 
      resp => {
        // need to add a dialog box, if user is ok, call pageRedirect
        if (resp.status === 303) {
          alert("Sorry, we haven't met yet.")
          pageRedirect(resp.responseText); 
        }
      }
    );
}

async function stg2(user,nonce) {
  let signature = await sign(nonce);

  let payload = 'stage=2&user='+ user +'&signed='+signature;
    $.post("/login",payload,"text")
      // Clean it later
      .always( () => { console.log("2: "+payload); })
      .done( 
        resp => {
          alert("Welcome ! Here is your cookie, don't forget the milk and go on ~:)");
          // Browser reject the redirection O_o
          pageRedirect(resp.responseText); 
        }
      )
      .fail(
        resp => {
          if (resp.status === 401)
            alert("Incorrect private key provided !");
          else
            alert("Unknown error returned.");
        }
      );
}
  
  function pageRedirect(url) {
      $(location).attr("href", url);
  }


/***********************************************
                  END
**********************************************/


/*   Defines   */
let elem = document.getElementById('authentificate');
if (elem != null) elem.addEventListener('click', authentificate, true);

elem = document.getElementById('generate');
if (elem != null) elem.addEventListener('click', generate, true);

elem = document.getElementById('verify')
if (elem != null) elem.addEventListener('click', verify, true);

elem = document.getElementById('sign')
if (elem != null) elem.addEventListener('click', test_sign, true);

  
  