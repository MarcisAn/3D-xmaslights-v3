// CanvasRecorder.js - smusamashah
// To record canvas effitiently using MediaRecorder
// https://webrtc.github.io/samples/src/content/capture/canvas-record/

export function CanvasRecorder(
  this: any,
  canvas: { captureStream: () => any },
  video_bits_per_sec: any
) {
  this.start = startRecording;
  this.stop = stopRecording;
  this.save = download;

  var recordedBlobs: any[] | undefined = [];
  var supportedType: string | null = null;
  var mediaRecorder: MediaRecorder | null = null;

  var stream = canvas.captureStream();
  if (typeof stream == undefined || !stream) {
    return;
  }

  const video = document.createElement("video");
  video.style.display = "none";

  function startRecording() {
    let types = [
      "video/webm",
      "video/webm,codecs=vp9",
      "video/vp8",
      "video/webm;codecs=vp8",
      "video/webm;codecs=daala",
      "video/webm;codecs=h264",
      "video/mpeg",
    ];

    for (let i in types) {
      if (MediaRecorder.isTypeSupported(types[i])) {
        supportedType = types[i];
        break;
      }
    }
    if (supportedType == null) {
      console.log("No supported type found for MediaRecorder");
    }
    let options = {
      mimeType: supportedType,
      videoBitsPerSecond: video_bits_per_sec || 2500000, // 2.5Mbps
    };

    recordedBlobs = [];
    try {
      //@ts-ignore
      mediaRecorder = new MediaRecorder(stream, options);
    } catch (e) {
      alert("MediaRecorder is not supported by this browser.");
      console.error("Exception while creating MediaRecorder:", e);
      return;
    }

    console.log(
      "Created MediaRecorder",
      mediaRecorder,
      "with options",
      options
    );
    mediaRecorder.onstop = handleStop;
    mediaRecorder.ondataavailable = handleDataAvailable;
    mediaRecorder.start(100); // collect 100ms of data blobs
    console.log("MediaRecorder started", mediaRecorder);
  }

  function handleDataAvailable(event: { data: { size: number } }) {
    if (event.data && event.data.size > 0) {
      //@ts-ignore
      recordedBlobs.push(event.data);
    }
  }

  function handleStop(event: any) {
    console.log("Recorder stopped: ", event);
    //@ts-ignore
    const superBuffer = new Blob(recordedBlobs, { type: supportedType });
    video.src = window.URL.createObjectURL(superBuffer);
  }

  function stopRecording() {
    //@ts-ignore
    mediaRecorder.stop();
    console.log("Recorded Blobs: ", recordedBlobs);
    video.controls = true;
  }

  function download(file_name: string) {
    const name = file_name || "recording.webm";
    //@ts-ignore
    const blob = new Blob(recordedBlobs, { type: supportedType });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.style.display = "none";
    a.href = url;
    a.download = name;
    document.body.appendChild(a);
    a.click();
    setTimeout(() => {
      document.body.removeChild(a);
      window.URL.revokeObjectURL(url);
    }, 100);
  }
}
