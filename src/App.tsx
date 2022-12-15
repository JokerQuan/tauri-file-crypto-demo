import React, { useEffect, useState, useRef } from 'react'
import reactLogo from './assets/react.svg'
import { invoke } from '@tauri-apps/api'
import { listen, once } from '@tauri-apps/api/event';
import './App.css'
import { Button, UploadFile, Space, Progress, Typography, message } from 'antd'
import { useAsyncEffect } from 'ahooks';

interface File {
  path: string,
  progress: number,
}

function App() {
  const [fileList, setFileList] = useState<File[]>([]);
  const progressList = useRef<File[]>([]);

  useEffect(() => {
    // (async () => {
    //   await listen("update-progress", async (e: any) => {
    //     console.log(fileList);
    //     const {index, progress, total} = e.payload;
    //     fileList[index].progress = Math.floor(progress / total) * 100;
    //     setFileList([...fileList]);
    //   });
    // })();

    // once("update-progress", (e: any) => {
    //   const {index, progress, total} = e.payload;
    //   fileList[index].progress = Math.floor(progress / total * 100);
    //   console.log(fileList[index].progress);
    //   setFileList([...fileList]);
    // })
    
  }, [fileList]);

  useAsyncEffect(async () => {
    const unlistener = await listen("update-progress", (e: any) => {
      const {index, progress, total} = e.payload;
      progressList.current[index].progress = Math.floor(progress / total * 100);
      // console.log(index, progressList.current[index].progress);
      setFileList([...progressList.current]);
    });
    // return unlistener;
  }, [])

  const filePick = () => {
    invoke('file_pick').then((response: any) => {
      if (!response || response.length == 0) return;
      progressList.current = response.map((file:any) => ({path: file}))
      setFileList(progressList.current)
    })
  }

  const encrypt = () => {
    invoke('encrypt', { paths: fileList.map(f => f.path) }).then((response) => {})
  }

  const decrypt = () => {
    let isAllEncrypt = fileList.every(f => f.path.match(/\.encrypt$/));
    if (isAllEncrypt) {
      invoke('decrypt', { paths: fileList.map(f => f.path) }).then((response) => {})
    } else {
      message.warning("所有文件应该以.encrypt为扩展名")
    }
  }

  const reset = () => {
    progressList.current = [];
    setFileList([])
  }

  return (
    <div className='app'>
      <Space className='btns'>
        <Button type='primary' onClick={filePick}>选择文件</Button>
        <Button type='primary' onClick={encrypt} disabled={fileList.length <= 0}>加密</Button>
        <Button type='primary' onClick={decrypt} disabled={fileList.length <= 0}>解密</Button>
        <Button onClick={reset}>重置</Button>
      </Space>
      <div className='progress-container'>
        {
          fileList.map((file, index) => (
            <div key={file.path}>
              <Typography.Text>{file.path}</Typography.Text>
              <Progress style={{opacity: file.progress > 0 ? 1 : 0}}
                percent={file.progress}
              />
            </div>
          ))
        }
      </div>
    </div>
  )
}

export default App