defmodule SmokeTest do
  require Logger

  def accept(port) do
    {:ok, socket} = :gen_tcp.listen(port, [:binary, packet: :raw, active: false, reuseaddr: true])

    Logger.info("Accepting connections on port #{port}")
    loop_acceptor(socket)
  end

  defp loop_acceptor(socket) do
    {:ok, client} = :gen_tcp.accept(socket)
    {:ok, pid} = Task.Supervisor.start_child(SmokeTest.TaskSupervisor, fn -> serve(client) end)

    case :gen_tcp.controlling_process(client, pid) do
      :ok ->
        Logger.info("ok")

      {:error, reason} ->
        Logger.info(inspect(reason))
    end

    loop_acceptor(socket)
  end

  defp serve(socket) do
    with {:ok, data} <- :gen_tcp.recv(socket, 0) do
      :gen_tcp.send(socket, data)
      serve(socket)
    else
      {:error, :closed} -> Logger.info("Socket closed")
    end
  end
end
