FROM mcr.microsoft.com/dotnet/runtime:8.0 AS base
WORKDIR /app

FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src
COPY ["codewars/codewars.csproj", "codewars/"]
RUN dotnet restore "codewars/codewars.csproj"
COPY . .
WORKDIR "/src/codewars"
RUN dotnet build "codewars.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "codewars.csproj" -c Release -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "codewars.dll"]
