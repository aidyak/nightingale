require_relative 'lib/nightingale'

# 1. ゲームループを別スレッドで回す
Thread.new do
  sleep(1)
  puts "ゲームエンジン起動！ 矢印キー（またはWASD）で赤い四角形を動かせます。"

  # Ruby側でプレイヤーの座標を管理
  x = 100.0
  y = 100.0
  speed = 4.0

  loop do
    # Rustに入力状態を問い合わせる
    x -= speed if key_down?(:left)
    x += speed if key_down?(:right)
    y -= speed if key_down?(:up)
    y += speed if key_down?(:down)

    # 画面外に出ないようにする簡単な画面端判定
    x = [[x, 0.0].max, 750.0].min
    y = [[y, 0.0].max, 550.0].min

    # 計算した新しい座標をRustのエンジンに送る
    update_box_position(x, y)
    
    sleep(1.0 / 60.0)
  end
end

# 2. メインスレッドで描画エンジンを起動
start_game_engine
