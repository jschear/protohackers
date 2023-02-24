defmodule BudgetChat do
  require Logger

  def accept(port) do
    {:ok, socket} =
      :gen_tcp.listen(port, [:binary, packet: :line, active: false, reuseaddr: true])

    Logger.info("Accepting connections on port #{port}")
    loop_acceptor(socket)
  end

  defp loop_acceptor(socket) do
    {:ok, client} = :gen_tcp.accept(socket)
    {:ok, pid} = Task.Supervisor.start_child(BudgetChat.TaskSupervisor, fn -> serve(client) end)

    case :gen_tcp.controlling_process(client, pid) do
      :ok ->
        Logger.info("ok")

      {:error, reason} ->
        Logger.info(inspect(reason))
    end

    loop_acceptor(socket)
  end

  defp serve(socket) do
  end
end
