// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_application_custom_export.h"

#include "custom/commands/write_random_things_command.h"

#include "repository/interface_brand_repository.h"
#include "repository/interface_car_repository.h"
#include "repository/interface_client_repository.h"
#include "repository/interface_passenger_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;

using namespace FrontEnds::Contracts::Repository;

using namespace FrontEnds::Contracts::CQRS::Custom::Commands;

namespace FrontEnds::Application::Features::Custom::Commands
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CUSTOM_EXPORT WriteRandomThingsCommandHandler : public QObject
{
    Q_OBJECT
  public:
    WriteRandomThingsCommandHandler(InterfaceCarRepository *carRepository,
                                    InterfacePassengerRepository *passengerRepository,
                                    InterfaceBrandRepository *brandRepository,
                                    InterfaceClientRepository *clientRepository);

    Result<void> handle(QPromise<Result<void>> &progressPromise, const WriteRandomThingsCommand &request);

    Result<void> restore();

  Q_SIGNALS:

    void writeRandomThingsChanged();

  private:
    InterfaceCarRepository *m_carRepository;
    InterfacePassengerRepository *m_passengerRepository;
    InterfaceBrandRepository *m_brandRepository;
    InterfaceClientRepository *m_clientRepository;
    Result<void> handleImpl(QPromise<Result<void>> &progressPromise, const WriteRandomThingsCommand &request);

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Custom::Commands