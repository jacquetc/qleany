// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_all_passenger_query_handler.h"
#include "repository/interface_passenger_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace FrontEnds::Application::Features::Passenger::Queries;

GetAllPassengerQueryHandler::GetAllPassengerQueryHandler(InterfacePassengerRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<PassengerDTO>> GetAllPassengerQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllPassengerQueryHandler::handle called";

    Result<QList<PassengerDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<PassengerDTO>>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllPassengerQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<QList<PassengerDTO>> GetAllPassengerQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllPassengerQueryHandler::handleImpl called";

    // do
    auto passengerResult = m_repository->getAll();

    QLN_RETURN_IF_ERROR(QList<PassengerDTO>, passengerResult)

    // map
    QList<PassengerDTO> dtoList;

    for (const FrontEnds::Entities::Passenger &passenger : passengerResult.value())
    {
        auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<FrontEnds::Entities::Passenger, PassengerDTO>(passenger);
        dtoList.append(dto);
    }

    qDebug() << "GetAllPassengerQueryHandler::handleImpl done";

    return Result<QList<PassengerDTO>>(dtoList);
}

bool GetAllPassengerQueryHandler::s_mappingRegistered = false;

void GetAllPassengerQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<FrontEnds::Entities::Passenger,
                                                           Contracts::DTO::Passenger::PassengerDTO>(true, true);
}