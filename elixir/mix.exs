defmodule ProtohackersElixir.MixProject do
  use Mix.Project

  def project do
    [
      apps_path: "apps",
      version: "0.1.0",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      releases: [
        smoke_test: [
          applications: [smoke_test: :permanent]
        ],
        means_to_an_end: [
          applications: [means_to_an_end: :permanent]
        ],
        budget_chat: [
          applications: [budget_chat: :permanent]
        ]
      ]
    ]
  end

  # Dependencies listed here are available only for this
  # project and cannot be accessed from applications inside
  # the apps folder.
  #
  # Run "mix help deps" for examples and options.
  defp deps do
    []
  end
end
