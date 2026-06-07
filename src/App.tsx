import { useState, useRef, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import './App.css'

type Emotion = 'normal' | 'happy' | 'lost' | 'sleepy' | 'sad'

const EMOTION_IMAGES: Record<Emotion, string> = {
  normal: '/Idle_Normal.png',
  happy: '/Idle_Jump.png',
  lost: '/Idle_Lost.png',
  sleepy: '/Idle_Rest.png',
  sad: '/Idle_Shadow.png',
}

const EMOTION_LABELS: Record<Emotion, string> = {
  normal: '呆滞',
  happy: '开心',
  lost: '迷茫',
  sleepy: '困倦',
  sad: '低落',
}

function App() {
  const [emotion, setEmotion] = useState<Emotion>('normal')
  const [input, setInput] = useState('')
  const [loading, setLoading] = useState(false)
  const [dragging, setDragging] = useState(false)
  const [messages, setMessages] = useState<{ role: string; text: string }[]>([
    { role: 'cat', text: '喵~ 你好呀，我是 Sharin！' },
  ])
  const chatEndRef = useRef<HTMLDivElement>(null)
  const dragTimer = useRef(0)

  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  function onCatMouseDown(e: React.MouseEvent) {
    if (e.button !== 0) return
    dragTimer.current = window.setTimeout(async () => {
      setDragging(true)
      try {
        await getCurrentWindow().startDragging()
      } catch {
        // fallback: OS drag might not work from timer on some platforms
      }
      setDragging(false)
    }, 300)
  }

  function onCatMouseUp() {
    clearTimeout(dragTimer.current)
    setDragging(false)
  }

  function onCatMouseLeave() {
    clearTimeout(dragTimer.current)
    setDragging(false)
  }

  async function handleSend() {
    const msg = input.trim()
    if (!msg || loading) return

    setInput('')
    setMessages((prev) => [...prev, { role: 'user', text: msg }])
    setLoading(true)

    try {
      const res = await invoke<{ reply: string; emotion: string }>('chat_with_cat', {
        message: msg,
      })
      const em = (['happy', 'normal', 'lost', 'sleepy', 'sad'].includes(res.emotion)
        ? res.emotion
        : 'normal') as Emotion
      setEmotion(em)
      setMessages((prev) => [...prev, { role: 'cat', text: res.reply }])
    } catch {
      setMessages((prev) => [...prev, { role: 'cat', text: '喵... 好像有点卡住了...' }])
    }
    setLoading(false)
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  return (
    <div className="app">
      <div className="top-bar">
        <span className="top-title">Sharin</span>
        <button className="close-btn" onClick={() => getCurrentWindow().close()}>x</button>
      </div>
      <div
        className={`cat-area${dragging ? ' dragging' : ''}`}
        onMouseDown={onCatMouseDown}
        onMouseUp={onCatMouseUp}
        onMouseLeave={onCatMouseLeave}
      >
        <img
          className={`cat-img ${emotion}`}
          src={EMOTION_IMAGES[emotion]}
          alt={EMOTION_LABELS[emotion]}
          draggable={false}
        />
        <span className="emotion-badge">[ {EMOTION_LABELS[emotion]} ]</span>
      </div>

      <div className="chat-area">
        <div className="messages" id="chat-scroll">
          {messages.map((m, i) =>
            m.role === 'cat' ? (
              <div key={i} className="msg-cat">
                <span className="msg-badge">Sharin</span>
                <p className="msg-text">{m.text}</p>
              </div>
            ) : (
              <div key={i} className="msg-user">
                <p className="msg-text">{m.text}</p>
              </div>
            )
          )}
          <div ref={chatEndRef} />
        </div>

        <div className="input-row">
          <input
            className="chat-input"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="对 Sharin 说点什么..."
            disabled={loading}
          />
          <button className="send-btn" onClick={handleSend} disabled={loading || !input.trim()}>
            {loading ? '...' : 'Send'}
          </button>
        </div>
      </div>
    </div>
  )
}

export default App
