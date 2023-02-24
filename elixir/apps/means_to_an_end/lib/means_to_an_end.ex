defmodule MeansToAnEnd do
  require Logger

  def accept(port) do
    {:ok, socket} = :gen_tcp.listen(port, [:binary, packet: :raw, active: false, reuseaddr: true])
    Logger.info("Accepting connections on port #{port}")
    loop_acceptor(socket)
  end

  defp loop_acceptor(socket) do
    {:ok, client} = :gen_tcp.accept(socket)

    {:ok, pid} =
      Task.Supervisor.start_child(MeansToAnEnd.TaskSupervisor, fn -> serve(client, %{}) end)

    with {:error, reason} <- :gen_tcp.controlling_process(client, pid) do
      Logger.info(inspect(reason))
    end

    loop_acceptor(socket)
  end

  defp serve(socket, map) do
    case :gen_tcp.recv(socket, 9) do
      {:ok, <<?I, timestamp::32-integer-signed, price::32-integer-signed>>} ->
        Logger.info("Insert: #{timestamp}, #{price}")
        map = Map.put(map, timestamp, price)
        serve(socket, map)

      {:ok, <<?Q, mintime::32-integer-signed, maxtime::32-integer-signed>>} ->
        Logger.info("Query: #{mintime}, #{maxtime}")

        average_price =
          cond do
            mintime >= maxtime ->
              0

            true ->
              {count, price_sum} =
                map
                |> Map.filter(fn {timestamp, _} ->
                  mintime <= timestamp and timestamp <= maxtime
                end)
                |> Enum.reduce(
                  {0, 0},
                  fn {_, price}, {acc_count, acc_sum} ->
                    {acc_count + 1, acc_sum + price}
                  end
                )

              Logger.info("Price sum, count: #{price_sum}, #{count}")

              case count do
                0 ->
                  0

                nonzero_count ->
                  floor(price_sum / nonzero_count)
              end
          end

        Logger.info("Average price: #{average_price}")

        :gen_tcp.send(socket, <<average_price::integer-signed-size(32)>>)
        serve(socket, map)

      {:error, reason} ->
        Logger.info(inspect(reason))

      anything ->
        Logger.info("Error parsing message: #{inspect(anything)}")
    end
  end
end
