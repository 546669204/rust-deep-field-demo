const { invoke } = window.__TAURI__.tauri;

function selectFile() {
  return new Promise((resolve, reject) => {
    let file = document.createElement("input")
    file.type = "file";
    file.style.cssText = `position: absolute;left: -9999px;`
    file.onchange = (e) => {
      resolve(file.files);
      file.remove();
    }
    document.body.appendChild(file);
    file.click();
  })
}

let txt;

window.openFile = async () => {
  const files = await selectFile();
  const reader = new FileReader();
  reader.addEventListener("load", async () => {
    const file_base64 = reader.result.split(',')[1];
    const result = await invoke("ppppp", {
      fileBytes: file_base64, splitLimit: [
        [0, 50],
        [51, 100],
        [101, 150],
        [151, 200],
        [201, 255],
      ]
    });

    const b = document.querySelector("#greet-msg");
    b.innerHTML = '';
    let zindex = 0;
    for (const ret of [file_base64, ...result]) {
      const img = new Image();
      img.src = "data:image/png;base64," + ret;
      img.style.position = "absolute";
      img.style.zIndex = zindex;
      zindex += 10;
      b.appendChild(img);
    }
    const big_img = await new Promise(r => {
      const img = new Image();
      img.onload = () => r(img);
      img.src = "data:image/png;base64," + file_base64;
    });

    b.style.width = big_img.width;
    b.style.height = big_img.height;
    
    txt = document.createElement("div");
    txt.style.zIndex = 31;


    b.appendChild(txt);
  })
  reader.readAsDataURL(files[0]);
}

function addPad(v) {
  return String(v).padStart(2, "0");
}
setInterval(() => {
  if (!txt) return;
  let m = new Date();
  txt.innerText = [
    [
      m.getFullYear(),
      m.getMonth() + 1,
      m.getDate()
    ].map(addPad).join("-"),
    [
      m.getHours(),
      m.getMinutes(),
      m.getSeconds()
    ].map(addPad).join(":")
  ].join("\n")
}, 1000);


