// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "simple_example_application_custom_export.h"

#include "custom/commands/close_system_command.h"

#include "repository/interface_brand_repository.h"
#include "repository/interface_car_repository.h"
#include "repository/interface_client_repository.h"
#include "repository/interface_passenger_repository.h"
#include "result.h"
#include <QPromise>

using namespace Simple;

using namespace Simple::Contracts::Repository;

using namespace Simple::Contracts::CQRS::Custom::Commands;

namespace Simple::Application::Features::Custom::Commands
{
class SIMPLE_EXAMPLE_APPLICATION_CUSTOM_EXPORT CloseSystemCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CloseSystemCommandHandler(InterfaceCarRepository *carRepository, InterfacePassengerRepository *passengerRepository,
                              InterfaceBrandRepository *brandRepository, InterfaceClientRepository *clientRepository);

    Result<void> handle(QPromise<Result<void>> &progressPromise, const CloseSystemCommand &request);

    Result<void> restore();

  Q_SIGNALS:

    void closeSystemChanged();

  private:
    InterfaceCarRepository *m_carRepository;
    InterfacePassengerRepository *m_passengerRepository;
    InterfaceBrandRepository *m_brandRepository;
    InterfaceClientRepository *m_clientRepository;
    Result<void> handleImpl(QPromise<Result<void>> &progressPromise, const CloseSystemCommand &request);

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Custom::Commands